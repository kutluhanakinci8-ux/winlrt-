import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import "./App.css";

type Format = "zip" | "wlrt" | "rar";
type Profile = "fast" | "balanced" | "maximum" | "email";

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

  useEffect(() => {
    invoke<EngineInfo>("engine_info")
      .then(setInfo)
      .catch(() => undefined);
  }, []);

  async function pickArchive() {
    setError("");
    const selected = await open({
      multiple: false,
      filters: [{ name: "Arşiv", extensions: ["zip", "wlrt", "rar"] }],
    });
    if (selected && typeof selected === "string") {
      setPath(selected);
      const ext = selected.split(".").pop()?.toLowerCase();
      if (ext === "wlrt" || ext === "zip" || ext === "rar") {
        setFormat(ext);
      }
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
    try {
      const res = await invoke<ListResult>("list_archive", {
        path,
        format,
        password: password || null,
      });
      setEntries(res.entries);
      setStatus(`${res.entries.length} girdi listelendi.`);
    } catch (e) {
      setEntries([]);
      setError(String(e));
    }
  }

  async function testArchive() {
    setError("");
    setStatus("");
    if (!path) {
      setError("Önce bir arşiv seçin.");
      return;
    }
    try {
      await invoke("test_archive", {
        path,
        format,
        password: password || null,
      });
      setStatus("Bütünlük testi: OK");
    } catch (e) {
      setError(String(e));
    }
  }

  async function extractArchive() {
    setError("");
    setStatus("");
    if (!path || !destDir) {
      setError("Arşiv ve hedef klasör gerekli.");
      return;
    }
    try {
      const res = await invoke<{ message: string }>("extract_archive", {
        path,
        dest: destDir,
        format,
        password: password || null,
      });
      setStatus(res.message);
    } catch (e) {
      setError(String(e));
    }
  }

  async function createArchive() {
    setError("");
    setStatus("");
    if (!destDir || !path) {
      setError("Kaynak klasör (hedef alanı) ve çıktı dosya yolu gerekli.");
      return;
    }
    try {
      const res = await invoke<{ message: string }>("create_archive", {
        source: destDir,
        dest: path,
        format,
        profile,
      });
      setStatus(res.message);
    } catch (e) {
      setError(String(e));
    }
  }

  return (
    <main className="app">
      <header className="header">
        <div>
          <p className="eyebrow">winlrt · Faz 2 beta</p>
          <h1>winlrtRAR</h1>
          <p className="sub">
            ZIP, .wlrt ve RAR5 açma; ZIP / .wlrt oluşturma. Profiller: hız,
            sıkıştırma, e-posta eki.
          </p>
        </div>
        {info && (
          <div className="meta">
            <span>{info.motor}</span>
            <span>{info.phase}</span>
          </div>
        )}
      </header>

      <section className="panel">
        <div className="row">
          <label>
            Format
            <select
              value={format}
              onChange={(e) => setFormat(e.target.value as Format)}
            >
              <option value="zip">ZIP</option>
              <option value="wlrt">.wlrt</option>
              <option value="rar">RAR (okuma)</option>
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
            RAR parola
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
    </main>
  );
}

export default App;
