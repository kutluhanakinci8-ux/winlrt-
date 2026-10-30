import { useCallback, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { open, save } from "@tauri-apps/plugin-dialog";
import "./App.css";

type Profile = "fast" | "balanced" | "maximum" | "email";

interface EngineInfo {
  product: string;
  version: string;
  motor: string;
  explorer_context_menu?: string;
}

interface ListResult {
  path: string;
  entries: string[];
}

interface CompressFolderHint {
  folder: string;
  suggested_output: string;
}

interface CompressSelectionHint {
  sources: string[];
  suggested_output: string;
}

export default function App() {
  const [info, setInfo] = useState<EngineInfo | null>(null);
  const [archivePath, setArchivePath] = useState<string | null>(null);
  const [entries, setEntries] = useState<string[]>([]);
  const [profile, setProfile] = useState<Profile>("email");
  const [status, setStatus] = useState<string>("");
  const [busy, setBusy] = useState(false);

  const loadArchive = useCallback(async (picked: string) => {
    setBusy(true);
    setStatus("");
    try {
      const res = await invoke<ListResult>("list_zip", { archive: picked, password: null });
      setArchivePath(res.path);
      setEntries(res.entries);
      setStatus(`${res.entries.length} öğe`);
    } catch (e) {
      setStatus(String(e));
    } finally {
      setBusy(false);
    }
  }, []);

  const runCreate = useCallback(
    async (source: string, dest: string) => {
      setBusy(true);
      try {
        const res = await invoke<{ message: string }>("create_zip", {
          source,
          dest,
          profile,
          password: null,
        });
        setStatus(res.message);
        await loadArchive(dest);
      } catch (e) {
        setStatus(String(e));
      } finally {
        setBusy(false);
      }
    },
    [loadArchive, profile],
  );

  const runCreateSelection = useCallback(
    async (sources: string[], dest: string) => {
      setBusy(true);
      try {
        const res = await invoke<{ message: string }>("create_zip_from_paths", {
          sources,
          dest,
          profile,
          password: null,
        });
        setStatus(res.message);
        await loadArchive(dest);
      } catch (e) {
        setStatus(String(e));
      } finally {
        setBusy(false);
      }
    },
    [loadArchive, profile],
  );

  useEffect(() => {
    invoke<EngineInfo>("engine_info").then(setInfo).catch(console.error);

    invoke<string[]>("startup_archives")
      .then((paths) => {
        if (paths[0]) void loadArchive(paths[0]);
      })
      .catch(() => undefined);

    invoke<CompressFolderHint | null>("startup_compress_folder")
      .then((hint) => {
        if (hint) void runCreate(hint.folder, hint.suggested_output);
      })
      .catch(() => undefined);

    invoke<CompressSelectionHint | null>("startup_compress_selection")
      .then((hint) => {
        if (hint) void runCreateSelection(hint.sources, hint.suggested_output);
      })
      .catch(() => undefined);

    const unlistenOpen = listen<string>("open-archive", (event) => {
      if (event.payload) void loadArchive(event.payload);
    });
    const unlistenDone = listen<string>("cli-action-done", (event) => {
      if (event.payload) setStatus(event.payload);
    });
    const unlistenErr = listen<string>("cli-action-error", (event) => {
      if (event.payload) setStatus(event.payload);
    });
    const unlistenCompress = listen<CompressFolderHint>("compress-folder", (event) => {
      if (event.payload) void runCreate(event.payload.folder, event.payload.suggested_output);
    });
    const unlistenSelection = listen<CompressSelectionHint>("compress-selection", (event) => {
      if (event.payload) {
        void runCreateSelection(event.payload.sources, event.payload.suggested_output);
      }
    });

    return () => {
      unlistenOpen.then((un) => un()).catch(() => undefined);
      unlistenDone.then((un) => un()).catch(() => undefined);
      unlistenErr.then((un) => un()).catch(() => undefined);
      unlistenCompress.then((un) => un()).catch(() => undefined);
      unlistenSelection.then((un) => un()).catch(() => undefined);
    };
  }, [loadArchive, runCreate, runCreateSelection]);

  const openZip = useCallback(async () => {
    const picked = await open({
      multiple: false,
      filters: [{ name: "ZIP", extensions: ["zip"] }],
    });
    if (!picked || Array.isArray(picked)) return;
    await loadArchive(picked);
  }, [loadArchive]);

  const extractZip = useCallback(async () => {
    if (!archivePath) return;
    const dest = await open({ directory: true, multiple: false });
    if (!dest || Array.isArray(dest)) return;
    setBusy(true);
    try {
      const res = await invoke<{ message: string }>("extract_zip", {
        archive: archivePath,
        dest,
        password: null,
      });
      setStatus(res.message);
    } catch (e) {
      setStatus(String(e));
    } finally {
      setBusy(false);
    }
  }, [archivePath]);

  const createZip = useCallback(async () => {
    const source = await open({ directory: true, multiple: false });
    if (!source || Array.isArray(source)) return;
    const dest = await save({
      filters: [{ name: "ZIP", extensions: ["zip"] }],
      defaultPath: "arsiv.zip",
    });
    if (!dest) return;
    await runCreate(source, dest);
  }, [runCreate]);

  const testZip = useCallback(async () => {
    if (!archivePath) return;
    setBusy(true);
    try {
      const res = await invoke<{ message: string }>("test_zip", {
        archive: archivePath,
        password: null,
      });
      setStatus(res.message);
    } catch (e) {
      setStatus(String(e));
    } finally {
      setBusy(false);
    }
  }, [archivePath]);

  return (
    <div className="app">
      <header>
        <h1>{info?.product ?? "winlrtZIP"}</h1>
        <p className="meta">{info?.motor}</p>
        {info?.explorer_context_menu && (
          <p className="meta explorer">{info.explorer_context_menu}</p>
        )}
      </header>
      <div className="toolbar">
        <button type="button" disabled={busy} onClick={() => void openZip()}>
          ZIP aç
        </button>
        <button type="button" disabled={busy || !archivePath} onClick={() => void extractZip()}>
          Çıkar
        </button>
        <button type="button" disabled={busy || !archivePath} onClick={() => void testZip()}>
          Test
        </button>
        <label>
          Profil{" "}
          <select
            value={profile}
            disabled={busy}
            onChange={(e) => setProfile(e.target.value as Profile)}
          >
            <option value="email">E-posta eki</option>
            <option value="fast">Hızlı</option>
            <option value="balanced">Dengeli</option>
            <option value="maximum">Maksimum</option>
          </select>
        </label>
        <button type="button" disabled={busy} onClick={() => void createZip()}>
          Klasörden ZIP
        </button>
      </div>
      {archivePath && <p className="path">{archivePath}</p>}
      <ul className="entries">
        {entries.map((e) => (
          <li key={e}>{e}</li>
        ))}
      </ul>
      {status && <p className="status">{status}</p>}
    </div>
  );
}
