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
  rar: {
    order: string[];
    embedded_libunrar: boolean;
    sidecar_unrar: boolean;
  };
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

interface PlatformInfo {
  os: string;
  explorer_context_menu: string;
}

interface CompressFolderHint {
  folder: string;
  suggested_output: string;
}

interface CompressSelectionHint {
  sources: string[];
  suggested_output: string;
}

interface PasswordFeedback {
  length: number;
  strength: string;
  ok_for_create: boolean;
  hint: string;
}

interface VolumeInfo {
  multipart: boolean;
  open_path: string;
  selected_path: string;
  format: string;
  hint: string;
  can_repair: boolean;
}

type AppTab = "archive" | "transition";

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
  const [createSelection, setCreateSelection] = useState<string[]>([]);
  const [createOutput, setCreateOutput] = useState("");
  const [openPassword, setOpenPassword] = useState("");
  const [createPassword, setCreatePassword] = useState("");
  const [encryptCreate, setEncryptCreate] = useState(false);
  const [showOpenPassword, setShowOpenPassword] = useState(false);
  const [showCreatePassword, setShowCreatePassword] = useState(false);
  const [createPwFeedback, setCreatePwFeedback] = useState<PasswordFeedback | null>(
    null,
  );
  const [entries, setEntries] = useState<string[]>([]);
  const [status, setStatus] = useState("");
  const [error, setError] = useState("");
  const [dragOver, setDragOver] = useState(false);
  const [jobs, setJobs] = useState<Job[]>([]);
  const [tab, setTab] = useState<AppTab>("archive");
  const [platform, setPlatform] = useState<PlatformInfo | null>(null);
  const [transitionStep, setTransitionStep] = useState(0);
  const [volumeInfo, setVolumeInfo] = useState<VolumeInfo | null>(null);
  const [repairDir, setRepairDir] = useState("");

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

  const applyCompressFolder = useCallback((hint: CompressFolderHint) => {
    setCreateSelection([]);
    setCreateSource(hint.folder);
    setCreateOutput(hint.suggested_output);
    setFormat("wlrt");
    setTab("archive");
    setStatus(
      "Klasör sıkıştırma hazır: profili seçin, gerekirse Kaydet… ile yolu değiştirin, Oluştur.",
    );
    setTransitionStep((s) => Math.max(s, 4));
  }, []);

  const applyCompressSelection = useCallback((hint: CompressSelectionHint) => {
    setCreateSelection(hint.sources);
    setCreateSource(hint.sources[0] ?? "");
    setCreateOutput(hint.suggested_output);
    setFormat("wlrt");
    setTab("archive");
    setStatus(
      `${hint.sources.length} öğe seçildi — profili seçin, gerekirse Kaydet… ile yolu değiştirin, Oluştur.`,
    );
    setTransitionStep((s) => Math.max(s, 4));
  }, []);

  useEffect(() => {
    invoke<EngineInfo>("engine_info")
      .then(setInfo)
      .catch(() => undefined);

    invoke<PlatformInfo>("platform_info")
      .then(setPlatform)
      .catch(() => undefined);

    invoke<string[]>("startup_archives")
      .then((paths) => {
        if (paths[0]) {
          applyArchivePath(paths[0]);
          setStatus("Arşiv çift tık / komut satırı ile açıldı.");
        }
      })
      .catch(() => undefined);

    invoke<CompressFolderHint | null>("startup_compress_folder")
      .then((hint) => {
        if (hint) applyCompressFolder(hint);
      })
      .catch(() => undefined);

    invoke<CompressSelectionHint | null>("startup_compress_selection")
      .then((hint) => {
        if (hint) applyCompressSelection(hint);
      })
      .catch(() => undefined);

    const unlistenOpen = listen<string>("open-archive", (event) => {
      if (event.payload) {
        applyArchivePath(event.payload);
        setTab("archive");
        setStatus("Arşiv otomatik yüklendi.");
      }
    });

    const unlistenDone = listen<string>("cli-action-done", (event) => {
      if (event.payload) {
        setTab("archive");
        setStatus(event.payload);
      }
    });

    const unlistenErr = listen<string>("cli-action-error", (event) => {
      if (event.payload) {
        setTab("archive");
        setError(event.payload);
      }
    });

    const unlistenCompress = listen<CompressFolderHint>(
      "compress-folder",
      (event) => {
        if (event.payload) applyCompressFolder(event.payload);
      },
    );

    const unlistenSelection = listen<CompressSelectionHint>(
      "compress-selection",
      (event) => {
        if (event.payload) applyCompressSelection(event.payload);
      },
    );

    return () => {
      unlistenOpen.then((un) => un()).catch(() => undefined);
      unlistenDone.then((un) => un()).catch(() => undefined);
      unlistenErr.then((un) => un()).catch(() => undefined);
      unlistenCompress.then((un) => un()).catch(() => undefined);
      unlistenSelection.then((un) => un()).catch(() => undefined);
    };
  }, [applyArchivePath, applyCompressFolder, applyCompressSelection]);

  useEffect(() => {
    if (!encryptCreate || !createPassword) {
      setCreatePwFeedback(null);
      return;
    }
    invoke<PasswordFeedback>("evaluate_password", {
      password: createPassword,
      forCreate: true,
    })
      .then(setCreatePwFeedback)
      .catch(() => setCreatePwFeedback(null));
  }, [createPassword, encryptCreate]);

  useEffect(() => {
    if (!archivePath || format !== "rar") {
      setVolumeInfo(null);
      return;
    }
    invoke<VolumeInfo>("inspect_archive", { path: archivePath, format })
      .then(setVolumeInfo)
      .catch(() => setVolumeInfo(null));
  }, [archivePath, format]);

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
      setCreateSelection([]);
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
        password: openPassword || null,
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
        password: openPassword || null,
      });
      setStatus("Bütünlük testi: OK");
      finishJob(jobId, true);
    } catch (e) {
      setError(String(e));
      finishJob(jobId, false);
    }
  }

  async function pickRepairDir() {
    const selected = await open({ directory: true, multiple: false });
    if (selected && typeof selected === "string") {
      setRepairDir(selected);
    }
  }

  async function repairArchive() {
    setError("");
    setStatus("");
    if (!archivePath || !repairDir) {
      setError("RAR onarım için arşiv ve hedef klasör gerekli.");
      return;
    }
    const jobId = pushJob("RAR onarım (recovery)");
    try {
      const res = await invoke<{ message: string }>("repair_archive", {
        path: archivePath,
        dest: repairDir,
        password: openPassword || null,
      });
      setStatus(res.message);
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
        password: openPassword || null,
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
    if (!createSource && createSelection.length === 0) {
      setError("Kaynak klasör veya Explorer seçimi gerekli.");
      return;
    }
    if (!createOutput) {
      setError("Kaydet konumu seçin (Kaydet…).");
      return;
    }
    if (encryptCreate) {
      if (!createPassword || createPassword.length < 8) {
        setError("Şifreli arşiv için en az 8 karakterlik parola girin.");
        return;
      }
      if (createPwFeedback && !createPwFeedback.ok_for_create) {
        setError("Parola politikası karşılanmıyor.");
        return;
      }
    }
    const jobId = pushJob(`Oluştur: ${createOutput.split(/[/\\]/).pop()}`);
    try {
      const res =
        createSelection.length > 0
          ? await invoke<{ message: string }>("create_archive_from_paths", {
              sources: createSelection,
              dest: createOutput,
              format: createFormat,
              profile,
              password: encryptCreate ? createPassword : null,
            })
          : await invoke<{ message: string }>("create_archive", {
              source: createSource,
              dest: createOutput,
              format: createFormat,
              profile,
              password: encryptCreate ? createPassword : null,
            });
      setStatus(res.message);
      finishJob(jobId, true);
    } catch (e) {
      setError(String(e));
      finishJob(jobId, false);
    }
  }

  const createFormat = CREATE_FORMATS.includes(format) ? format : "zip";

  async function openDefaultApps() {
    setError("");
    try {
      await invoke("open_windows_default_apps");
      setTransitionStep((s) => Math.max(s, 2));
      setStatus("Windows varsayılan uygulamalar açıldı.");
    } catch (e) {
      setError(String(e));
    }
  }

  const transitionSteps = [
    "Beta kurulumu (winlrt.com.tr → İndir → GitHub Releases).",
    "Kurulumu tamamlayın; Explorer’da arşive sağ tık → winlrtRAR menüleri görünür.",
    "Varsayılan uygulama: ZIP/RAR için winlrtRAR’ı seçin (isteğe bağlı).",
    "Çift tık veya «buraya aç» ile günlük iş akışını test edin.",
    "Klasöre sağ tık → sıkıştır / hızlı .wlrt; çoklu dosyada «seçilenleri sıkıştır».",
  ];

  return (
    <main className="app">
      <header className="header">
        <div>
          <p className="eyebrow">winlrt · Faz 3 geçiş</p>
          <h1>winlrtRAR</h1>
          <p className="sub">
            Açma: ZIP, .wlrt, RAR, 7z · Oluşturma: ZIP / .wlrt · Explorer sağ
            tık (Windows kurulum).
          </p>
          <nav className="tabs" aria-label="Ana sekme">
            <button
              type="button"
              className={`tab ${tab === "archive" ? "active" : ""}`}
              onClick={() => setTab("archive")}
            >
              Arşiv
            </button>
            <button
              type="button"
              className={`tab ${tab === "transition" ? "active" : ""}`}
              onClick={() => setTab("transition")}
            >
              Geçiş sihirbazı
            </button>
          </nav>
        </div>
        {info && (
          <div className="meta">
            <span>{info.motor}</span>
            <span>{info.phase}</span>
            <span title={info.rar.order.join(" → ")}>
              RAR:{" "}
              {info.rar.embedded_libunrar
                ? "libunrar gömülü"
                : "CLI / rar crate"}
            </span>
          </div>
        )}
      </header>

      {tab === "transition" && (
        <section className="panel transition wizard">
          <h2>WinRAR → winlrtRAR</h2>
          <p className="muted">
            {platform?.explorer_context_menu ??
              "Explorer menüsü Windows NSIS kurulumunda eklenir."}
          </p>
          <ol className="transition-list wizard-steps">
            {transitionSteps.map((text, i) => (
              <li key={text} className={i <= transitionStep ? "done" : ""}>
                {text}
              </li>
            ))}
          </ol>
          <div className="row">
            <button
              type="button"
              className="btn primary"
              onClick={() => {
                setTransitionStep((s) => Math.max(s, 1));
                setTab("archive");
                setStatus("Arşiv sekmesinde çift tık / sürükle-bırak test edin.");
              }}
            >
              Adım 4 — arşiv sekmesine geç
            </button>
            <button type="button" className="btn" onClick={openDefaultApps}>
              Varsayılan uygulamalar (Windows)
            </button>
          </div>
        </section>
      )}

      {tab === "archive" && (
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
            Parola (aç / listele)
            <div className="password-row">
              <input
                type={showOpenPassword ? "text" : "password"}
                value={openPassword}
                onChange={(e) => setOpenPassword(e.target.value)}
                placeholder="ZIP / .wlrt / RAR / 7z"
                autoComplete="off"
              />
              <button
                type="button"
                className="btn tiny"
                onClick={() => setShowOpenPassword((v) => !v)}
              >
                {showOpenPassword ? "Gizle" : "Göster"}
              </button>
            </div>
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
        {volumeInfo?.hint && (
          <p className="status ok volume-hint">{volumeInfo.hint}</p>
        )}
        {format === "rar" && volumeInfo?.can_repair && (
          <div className="row repair-row">
            <button type="button" className="btn" onClick={pickRepairDir}>
              Onarım — hedef klasör…
            </button>
            <button type="button" className="btn" onClick={repairArchive}>
              RAR onarım dene
            </button>
            <span className="muted inline">UnRAR recovery (beta)</span>
          </div>
        )}
        {repairDir && <p className="path">Onarım hedefi: {repairDir}</p>}

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
            Çıktı: <strong>{createFormat}</strong>
          </span>
        </div>
        <div className="row security-row">
          <label className="checkbox-label">
            <input
              type="checkbox"
              checked={encryptCreate}
              onChange={(e) => setEncryptCreate(e.target.checked)}
            />
            Arşivi parola ile şifrele
          </label>
          {encryptCreate && (
            <label className="grow">
              Oluşturma parolası
              <div className="password-row">
                <input
                  type={showCreatePassword ? "text" : "password"}
                  value={createPassword}
                  onChange={(e) => setCreatePassword(e.target.value)}
                  placeholder="Min. 8 karakter"
                  autoComplete="new-password"
                />
                <button
                  type="button"
                  className="btn tiny"
                  onClick={() => setShowCreatePassword((v) => !v)}
                >
                  {showCreatePassword ? "Gizle" : "Göster"}
                </button>
              </div>
            </label>
          )}
        </div>
        {encryptCreate && createPwFeedback && (
          <p className={`pw-feedback strength-${createPwFeedback.strength}`}>
            Güç: {createPwFeedback.strength} · {createPwFeedback.hint}
          </p>
        )}
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
        {createSelection.length > 1 && (
          <p className="path">
            Explorer seçimi: {createSelection.length} öğe (ilk:{" "}
            {createSelection[0]?.split(/[/\\]/).pop()})
          </p>
        )}
        {createSource && createSelection.length <= 1 && (
          <p className="path">Kaynak: {createSource}</p>
        )}
        {createOutput && <p className="path">Çıktı: {createOutput}</p>}

        {status && <p className="status ok">{status}</p>}
        {error && <p className="status err">{error}</p>}
      </section>
      )}

      {tab === "archive" && (
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
      )}
    </main>
  );
}

export default App;
