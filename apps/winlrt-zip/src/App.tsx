import { useCallback, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open, save } from "@tauri-apps/plugin-dialog";
import "./App.css";

type Profile = "fast" | "balanced" | "maximum" | "email";

interface EngineInfo {
  product: string;
  version: string;
  motor: string;
}

interface ListResult {
  path: string;
  entries: string[];
}

export default function App() {
  const [info, setInfo] = useState<EngineInfo | null>(null);
  const [archivePath, setArchivePath] = useState<string | null>(null);
  const [entries, setEntries] = useState<string[]>([]);
  const [profile, setProfile] = useState<Profile>("email");
  const [status, setStatus] = useState<string>("");
  const [busy, setBusy] = useState(false);

  useEffect(() => {
    invoke<EngineInfo>("engine_info").then(setInfo).catch(console.error);
  }, []);

  const openZip = useCallback(async () => {
    const picked = await open({
      multiple: false,
      filters: [{ name: "ZIP", extensions: ["zip"] }],
    });
    if (!picked || Array.isArray(picked)) return;
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
    setBusy(true);
    try {
      const res = await invoke<{ message: string }>("create_zip", {
        source,
        dest,
        profile,
        password: null,
      });
      setStatus(res.message);
      setArchivePath(dest);
      const list = await invoke<ListResult>("list_zip", { archive: dest, password: null });
      setEntries(list.entries);
    } catch (e) {
      setStatus(String(e));
    } finally {
      setBusy(false);
    }
  }, [profile]);

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
