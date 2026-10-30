import { useCallback, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { open } from "@tauri-apps/plugin-dialog";
import "./App.css";

type Format = "zip" | "wlrt" | "rar" | "7z";
type Profile = "fast" | "balanced" | "maximum" | "email";
type JobStatus = "pending" | "running" | "done" | "error";

interface EngineInfo {
  product: string;
  phase: string;
  motor: string;
}

interface ListResult {
  entries: string[];
  format: string;
  path: string;
}

interface Job {
  id: string;
  label: string;
  status: JobStatus;
}

function formatFromPath(filePath: string): Format | null {
  const ext = filePath.split(".").pop()?.toLowerCase();
  if (ext === "zip" || ext === "wlrt" || ext === "rar" || ext === "7z") {
    return ext;
  }
  return null;
}

function App() {
  const [info, setInfo] = useState<EngineInfo | null>(null);
  const [format, setFormat] = useState<Format>("zip");
  const [profile, setProfile] = useState<Profile>("balanced");
  const [path, setPath] = useState("");
  const [destDir, setDestDir] = useState("");
  const [password, setPassword] = useState("");
  const [entries, setEntries] = useState<string[]>([]);
  const [status, setStatus] = useState("");
  const [error, setError] = useState("");
  const [dragOver, setDragOver] = useState(false);
  const [jobs, setJobs] = useState<Job[]>([]);

  const pushJob = useCallback((label: string, status: JobStatus = "running") => {
    const id = `${Date.now()}-${Math.random().toString(36).slice(2, 8)}`;
    setJobs((prev) => [{ id, label, status }, ...prev].slice(0, 12));
    return id;
  }, []);

  const finishJob = useCallback((id: string, ok: boolean) => {
    setJobs((prev) =>
      prev.map((j) =>
        j.id === id ? { ...j, status: ok ? "done" : "error" } : j,
      ),
    );
  }, []);

  const applyArchivePath = useCallback((filePath: string) => {
    setPath(filePath);
    const f = formatFromPath(filePath);
    if (f) setFormat(f);
  }, []);

  useEffect(() => {
    invoke<EngineInfo>("engine_info")
      .then(setInfo)
      .catch(() => undefined);
  }, []);

  useEffect(() => {
    let unlisten: (() => void) | undefined;
    getCurrentWindow()
      .onDragDropEvent((event) => {
        const payload = event.payload;
        if (payload.type === "over") {
          setDragOver(true);
        } else if (payload.type === "drop") {
          setDragOver(false);
          const dropped = payload.paths[0];
          if (dropped) {
            applyArchivePath(dropped);
            setStatus("Arşiv sürüklenerek seçildi.");
          }
        } else {
          setDragOver(false);
        }
      })
      .then((fn) => {
        unlisten = fn;
      })
      .catch(() => undefined);
    return () => {
      unlisten?.();
    };
  }, [applyArchivePath]);

  async function pickArchive() {
    setError("");
    const selected = await open({
      multiple: false,
      filters: [
        { name: "Arşiv", extensions: ["zip", "wlrt", "rar", "7z"] },
      ],
    });
    if (selected && typeof selected === "string") {
      applyArchivePath(selected);
    }
  }

  async function pickDest() {
    const selected = await open({ directory: true, multiple: false });
    if (selected && typeof selected === "string") {
      setDestDir(selected);
    }
  }

  async function pickSourceForCreate() {
    const selected = await open({ directory: true, multiple: false });
    if (selected && typeof selected === "string") {
      setDestDir(selected);
    }
  }

  async function listArchive() {
    setError("");
    setStatus("");
    if (!path) {
      setError("Önce bir arşiv seçin.");
      return;
    }
    const jobId = pushJob(`Listele: ${path.split(/[/\\]/).pop()}`);
    try {
      const res = await invoke<ListResult>("list_archive", {
        path,
        format,
        password: password || null,
      });
      setEntries(res.entries);
      setStatus(`${res.entries.length} girdi listelendi.`);
      finishJob(jobId, true);
    } catch (e) {
      setEntries([]);
      setError(String(e));
      finishJob(jobId, false);
    }
  }

  async function testArchive() {
    setError("");
    setStatus("");
    if (!path) {
      setError("Önce bir arşiv seçin.");
      return;
    }
    const jobId = pushJob(`Test: ${path.split(/[/\\]/).pop()}`);
    try {
      await invoke("test_archive", {
        path,
        format,
        password: password || null,
      });
      setStatus("Bütünlük testi: OK");
      finishJob(jobId, true);
    } catch (e) {
      setError(String(e));
      finishJob(jobId, false);
    }
  }

  async function extractArchive() {
    setError("");
    setStatus("");
    if (!path || !destDir) {
      setError("Arşiv ve hedef klasör gerekli.");
      return;
    }
    const jobId = pushJob(`Aç: ${path.split(/[/\\]/).pop()}`);
    try {
      const res = await invoke<{ message: string }>("extract_archive", {
        path,
        dest: destDir,
        format,
        password: password || null,
      });
      setStatus(res.message);
      finishJob(jobId, true);
    } catch (e) {
      setError(String(e));
      finishJob(jobId, false);
    }
  }

  async function createArchive() {
    setError("");
    setStatus("");
    if (!destDir || !path) {
      setError("Kaynak klasör ve çıktı dosya yolu gerekli.");
      return;
    }
    const jobId = pushJob(`Oluştur: ${path.split(/[/\\]/).pop()}`);
    try {
      const res = await invoke<{ message: string }>("create_archive", {
        source: destDir,
        dest: path,
        format,
        profile,
      });
      setStatus(res.message);
      finishJob(jobId, true);
    } catch (e) {
      setError(String(e));
      finishJob(jobId, false);
    }
  }

  return (
    <main className="app">
      <header className="header">
        <div>
          <p className="eyebrow">winlrt · Faz 2 beta</p>
          <h1>winlrtRAR</h1>
          <p className="sub">
            ZIP, .wlrt, RAR5 ve 7z açma; ZIP / .wlrt oluşturma. Arşivi pencereye
            sürükleyebilirsiniz.
          </p>
        </div>
        {info && (
          <div className="meta">
            <span>{info.motor}</span>
            <span>{info.phase}</span>
          </div>
        )}
      </header>

      <section className={`panel drop-zone ${dragOver ? "drag-over" : ""}`}>
        <p className="drop-hint">Arşivi buraya sürükleyin veya dosya seçin</p>
        <div className="row">
          <label>
            Format
            <select
              value={format}
              onChange={(e) => setFormat(e.target.value as Format)}
            >
              <option value="zip">ZIP</option>
              <option value="wlrt">.wlrt</option>
              <option value="rar">RAR</option>
              <option value="7z">7z</option>
            </select>
          </label>
          <label>
            Profil (oluştur)
            <select
              value={profile}
              onChange={(e) => setProfile(e.target.value as Profile)}
            >
              <option value="fast">Hızlı</option>
              <option value="balanced">Dengeli</option>
              <option value="maximum">Maksimum</option>
              <option value="email">E-posta eki</option>
            </select>
          </label>
          <label className="grow">
            Parola (RAR / 7z)
            <input
              type="password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              placeholder="Gerekirse"
            />
          </label>
        </div>
        <div className="row">
          <button type="button" className="btn primary" onClick={pickArchive}>
            Arşiv seç…
          </button>
          <button type="button" className="btn" onClick={pickDest}>
            Hedef klasör…
          </button>
          <button type="button" className="btn" onClick={listArchive}>
            Listele
          </button>
          <button type="button" className="btn" onClick={testArchive}>
            Test
          </button>
          <button type="button" className="btn" onClick={extractArchive}>
            Aç
          </button>
        </div>
        <div className="row">
          <button type="button" className="btn" onClick={pickSourceForCreate}>
            Oluştur — kaynak klasör…
          </button>
          <button type="button" className="btn primary" onClick={createArchive}>
            Oluştur (çıktı = arşiv yolu)
          </button>
        </div>
        {path && <p className="path">Arşiv / çıktı: {path}</p>}
        {destDir && <p className="path">Klasör: {destDir}</p>}
        {status && <p className="status ok">{status}</p>}
        {error && <p className="status err">{error}</p>}
      </section>

      <div className="split">
        <section className="panel list">
          <h2>İçerik</h2>
          {entries.length === 0 ? (
            <p className="muted">Henüz girdi yok.</p>
          ) : (
            <ul>
              {entries.map((name) => (
                <li key={name}>{name}</li>
              ))}
            </ul>
          )}
        </section>

        <section className="panel list">
          <h2>İş kuyruğu</h2>
          {jobs.length === 0 ? (
            <p className="muted">Son işlemler burada görünür.</p>
          ) : (
            <ul className="jobs">
              {jobs.map((j) => (
                <li key={j.id} className={`job job-${j.status}`}>
                  {j.label}
                </li>
              ))}
            </ul>
          )}
        </section>
      </div>
    </main>
  );
}

export default App;
