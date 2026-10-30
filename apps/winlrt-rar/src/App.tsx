import { useCallback, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { open, save } from "@tauri-apps/plugin-dialog";
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

const CREATE_FORMATS: Format[] = ["zip", "wlrt"];

function formatFromPath(filePath: string): Format | null {
  const ext = filePath.split(".").pop()?.toLowerCase();
  if (ext === "zip" || ext === "wlrt" || ext === "rar" || ext === "7z") {
    return ext;
  }
  return null;
}

function saveExtension(format: Format): string {
  if (format === "wlrt") return "wlrt";
  return format;
}

function App() {
  const [info, setInfo] = useState<EngineInfo | null>(null);
  const [format, setFormat] = useState<Format>("zip");
  const [profile, setProfile] = useState<Profile>("balanced");
  const [archivePath, setArchivePath] = useState("");
  const [extractDir, setExtractDir] = useState("");
  const [createSource, setCreateSource] = useState("");
  const [createOutput, setCreateOutput] = useState("");
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
    setArchivePath(filePath);
    const f = formatFromPath(filePath);
    if (f) setFormat(f);
    setEntries([]);
  }, []);

  useEffect(() => {
    invoke<EngineInfo>("engine_info")
      .then(setInfo)
      .catch(() => undefined);

    invoke<string[]>("startup_archives")
      .then((paths) => {
        if (paths[0]) {
          applyArchivePath(paths[0]);
          setStatus("Arşiv çift tık / komut satırı ile açıldı.");
        }
      })
      .catch(() => undefined);

    const unlistenPromise = listen<string>("open-archive", (event) => {
      if (event.payload) {
        applyArchivePath(event.payload);
        setStatus("Arşiv otomatik yüklendi.");
      }
    });

    return () => {
      unlistenPromise.then((un) => un()).catch(() => undefined);
    };
  }, [applyArchivePath]);

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

  async function pickExtractDir() {
    const selected = await open({ directory: true, multiple: false });
    if (selected && typeof selected === "string") {
      setExtractDir(selected);
    }
  }

  async function pickCreateSource() {
    const selected = await open({ directory: true, multiple: false });
    if (selected && typeof selected === "string") {
      setCreateSource(selected);
    }
  }

  async function pickCreateOutput() {
    const createFormat = CREATE_FORMATS.includes(format) ? format : "zip";
    const ext = saveExtension(createFormat);
    const selected = await save({
      title: "Arşiv kaydet",
      filters: [{ name: createFormat.toUpperCase(), extensions: [ext] }],
      defaultPath: `winlrt.${ext}`,
    });
    if (selected) {
      setCreateOutput(selected);
    }
  }

  async function listArchive() {
    setError("");
    setStatus("");
    if (!archivePath) {
      setError("Önce bir arşiv seçin.");
      return;
    }
    const jobId = pushJob(`Listele: ${archivePath.split(/[/\\]/).pop()}`);
    try {
      const res = await invoke<ListResult>("list_archive", {
        path: archivePath,
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
    if (!archivePath) {
      setError("Önce bir arşiv seçin.");
      return;
    }
    const jobId = pushJob(`Test: ${archivePath.split(/[/\\]/).pop()}`);
    try {
      await invoke("test_archive", {
        path: archivePath,
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
    if (!archivePath || !extractDir) {
      setError("Arşiv ve hedef klasör gerekli.");
      return;
    }
    const jobId = pushJob(`Aç: ${archivePath.split(/[/\\]/).pop()}`);
    try {
      const res = await invoke<{ message: string }>("extract_archive", {
        path: archivePath,
        dest: extractDir,
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
    const createFormat = CREATE_FORMATS.includes(format) ? format : "zip";
    if (!createSource) {
      setError("Kaynak klasör seçin.");
      return;
    }
    if (!createOutput) {
      setError("Kaydet konumu seçin (Kaydet…).");
      return;
    }
    const jobId = pushJob(`Oluştur: ${createOutput.split(/[/\\]/).pop()}`);
    try {
      const res = await invoke<{ message: string }>("create_archive", {
        source: createSource,
        dest: createOutput,
        format: createFormat,
        profile,
      });
      setStatus(res.message);
      finishJob(jobId, true);
    } catch (e) {
      setError(String(e));
      finishJob(jobId, false);
    }
  }

  const createFormat = CREATE_FORMATS.includes(format) ? format : "zip";

  return (
    <main className="app">
      <header className="header">
        <div>
          <p className="eyebrow">winlrt · Faz 2 beta</p>
          <h1>winlrtRAR</h1>
          <p className="sub">
            Açma: ZIP, .wlrt, RAR, 7z · Oluşturma: ZIP / .wlrt (profil + Kaydet
            diyalogu). Kurulum sonrası çift tık desteklenir.
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
        <h3 className="panel-title">Aç / listele</h3>
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
          <button type="button" className="btn" onClick={pickExtractDir}>
            Aç — hedef klasör…
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
        {archivePath && <p className="path">Arşiv: {archivePath}</p>}
        {extractDir && <p className="path">Aç hedefi: {extractDir}</p>}

        <h3 className="panel-title">Oluştur (ZIP / .wlrt)</h3>
        <div className="row">
          <label>
            Profil
            <select
              value={profile}
              onChange={(e) => setProfile(e.target.value as Profile)}
            >
              <option value="fast">Hızlı</option>
              <option value="balanced">Dengeli</option>
              <option value="maximum">Maksimum sıkıştırma</option>
              <option value="email">E-posta eki</option>
            </select>
          </label>
          <span className="muted inline">
            Çıktı formatı: <strong>{createFormat}</strong> (RAR/7z oluşturma yok)
          </span>
        </div>
        <div className="row">
          <button type="button" className="btn" onClick={pickCreateSource}>
            Kaynak klasör…
          </button>
          <button type="button" className="btn" onClick={pickCreateOutput}>
            Kaydet…
          </button>
          <button type="button" className="btn primary" onClick={createArchive}>
            Oluştur
          </button>
        </div>
        {createSource && <p className="path">Kaynak: {createSource}</p>}
        {createOutput && <p className="path">Çıktı: {createOutput}</p>}

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

      <section className="panel transition">
        <h2>WinRAR’dan geçiş (beta)</h2>
        <ol className="transition-list">
          <li>Beta kurulum: winlrt.com.tr → İndir → GitHub Releases (.exe).</li>
          <li>Kurulum zip / rar / 7z / wlrt ilişkilendirmesini kaydeder; çift tıkla test edin.</li>
          <li>Yeni arşivler için ZIP veya .wlrt oluşturun (profil + Kaydet).</li>
          <li>
            Tam sihirbaz ve Explorer sağ tık — Faz 3 (bkz. docs/WINLRTRAR_FAZ3_TRANSITION.md).
          </li>
        </ol>
      </section>
    </main>
  );
}

export default App;
