import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import "./App.css";

type Format = "zip" | "wlrt" | "rar";

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
  const [path, setPath] = useState("");
  const [entries, setEntries] = useState<string[]>([]);
  const [status, setStatus] = useState("");
  const [error, setError] = useState("");

  async function loadInfo() {
    setInfo(await invoke<EngineInfo>("engine_info"));
  }

  useEffect(() => {
    loadInfo().catch(() => undefined);
  }, []);

  async function pickArchive() {
    setError("");
    const selected = await open({
      multiple: false,
      filters: [
        {
          name: "Arşiv",
          extensions: ["zip", "wlrt", "rar"],
        },
      ],
    });
    if (selected && typeof selected === "string") {
      setPath(selected);
      const ext = selected.split(".").pop()?.toLowerCase();
      if (ext === "wlrt" || ext === "zip" || ext === "rar") {
        setFormat(ext);
      }
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
      const res = await invoke<ListResult>("list_archive", { path, format });
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
      await invoke("test_archive", { path, format });
      setStatus("Bütünlük testi: OK");
    } catch (e) {
      setError(String(e));
    }
  }

  return (
    <main className="app">
      <header className="header">
        <div>
          <p className="eyebrow">winlrt · Faz 2 beta iskelet</p>
          <h1>winlrtRAR</h1>
          <p className="sub">
            Yerli motor — ZIP ve .wlrt destekli; RAR okuma Faz 2 sprint’inde.
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
              <option value="rar">RAR (yakında)</option>
            </select>
          </label>
          <button type="button" className="btn primary" onClick={pickArchive}>
            Arşiv seç…
          </button>
          <button type="button" className="btn" onClick={listArchive}>
            Listele
          </button>
          <button type="button" className="btn" onClick={testArchive}>
            Test
          </button>
        </div>
        {path && <p className="path">{path}</p>}
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
