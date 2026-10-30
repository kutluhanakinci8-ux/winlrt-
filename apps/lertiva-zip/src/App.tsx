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

interface PasswordFeedback {
  ok_for_create: boolean;
  strength: string;
  hint: string;
}

interface PasswordPolicyView {
  id: string;
  label: string;
  min_length: number;
}

interface SecureCreateWizard {
  suggested_output: string;
  folder: string | null;
  sources: string[];
  policy: PasswordPolicyView;
}

interface CompressFolderHint {
  folder: string;
  suggested_output: string;
  secure_create: boolean;
}

interface CompressSelectionHint {
  sources: string[];
  suggested_output: string;
  secure_create: boolean;
}

interface WinzipMigrationGuide {
  winzip_detected: boolean;
  winzip_path: string | null;
  recommended_profile: string;
  steps: string[];
  feature_notes: string[];
}

export default function App() {
  const [info, setInfo] = useState<EngineInfo | null>(null);
  const [archivePath, setArchivePath] = useState<string | null>(null);
  const [entries, setEntries] = useState<string[]>([]);
  const [profile, setProfile] = useState<Profile>("email");
  const [status, setStatus] = useState<string>("");
  const [busy, setBusy] = useState(false);
  const [openPassword, setOpenPassword] = useState("");
  const [encryptCreate, setEncryptCreate] = useState(false);
  const [createPassword, setCreatePassword] = useState("");
  const [secureWizard, setSecureWizard] = useState<SecureCreateWizard | null>(null);
  const [wizardPassword, setWizardPassword] = useState("");
  const [wizardConfirm, setWizardConfirm] = useState("");
  const [wizardFeedback, setWizardFeedback] = useState<PasswordFeedback | null>(null);
  const [migrationGuide, setMigrationGuide] = useState<WinzipMigrationGuide | null>(null);
  const [migrationOpen, setMigrationOpen] = useState(false);

  const loadArchive = useCallback(
    async (picked: string, password?: string | null) => {
      setBusy(true);
      setStatus("");
      try {
        const pw = password === undefined ? openPassword || null : password;
        const res = await invoke<ListResult>("list_zip", { archive: picked, password: pw || null });
        setArchivePath(res.path);
        setEntries(res.entries);
        setStatus(`${res.entries.length} öğe`);
      } catch (e) {
        setStatus(String(e));
      } finally {
        setBusy(false);
      }
    },
    [openPassword],
  );

  const runCreate = useCallback(
    async (source: string, dest: string, password?: string | null) => {
      setBusy(true);
      try {
        const pw = password ?? (encryptCreate ? createPassword : null);
        const res = await invoke<{ message: string }>("create_zip", {
          source,
          dest,
          profile,
          password: pw || null,
        });
        setStatus(res.message);
        await loadArchive(dest, pw || null);
      } catch (e) {
        setStatus(String(e));
      } finally {
        setBusy(false);
      }
    },
    [createPassword, encryptCreate, loadArchive, profile],
  );

  const runCreateSelection = useCallback(
    async (sources: string[], dest: string, password?: string | null) => {
      setBusy(true);
      try {
        const pw = password ?? (encryptCreate ? createPassword : null);
        const res = await invoke<{ message: string }>("create_zip_from_paths", {
          sources,
          dest,
          profile,
          password: pw || null,
        });
        setStatus(res.message);
        await loadArchive(dest, pw || null);
      } catch (e) {
        setStatus(String(e));
      } finally {
        setBusy(false);
      }
    },
    [createPassword, encryptCreate, loadArchive, profile],
  );

  const applyCompressFolder = useCallback(
    (hint: CompressFolderHint) => {
      if (hint.secure_create) return;
      void runCreate(hint.folder, hint.suggested_output);
    },
    [runCreate],
  );

  const applyCompressSelection = useCallback(
    (hint: CompressSelectionHint) => {
      if (hint.secure_create) return;
      void runCreateSelection(hint.sources, hint.suggested_output);
    },
    [runCreateSelection],
  );

  useEffect(() => {
    if (!secureWizard || !wizardPassword) {
      setWizardFeedback(null);
      return;
    }
    invoke<PasswordFeedback>("evaluate_create_password", {
      password: wizardPassword,
      secureCreate: true,
    })
      .then(setWizardFeedback)
      .catch(() => setWizardFeedback(null));
  }, [wizardPassword, secureWizard]);

  useEffect(() => {
    invoke<EngineInfo>("engine_info").then(setInfo).catch(console.error);

    invoke<string[]>("startup_archives")
      .then((paths) => {
        if (paths[0]) void loadArchive(paths[0]);
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

    invoke<SecureCreateWizard | null>("startup_secure_wizard")
      .then((wizard) => {
        if (wizard) setSecureWizard(wizard);
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
      if (event.payload) applyCompressFolder(event.payload);
    });
    const unlistenSelection = listen<CompressSelectionHint>("compress-selection", (event) => {
      if (event.payload) applyCompressSelection(event.payload);
    });
    const unlistenSecure = listen<SecureCreateWizard>("secure-create-wizard", (event) => {
      if (event.payload) setSecureWizard(event.payload);
    });

    return () => {
      unlistenOpen.then((un) => un()).catch(() => undefined);
      unlistenDone.then((un) => un()).catch(() => undefined);
      unlistenErr.then((un) => un()).catch(() => undefined);
      unlistenCompress.then((un) => un()).catch(() => undefined);
      unlistenSelection.then((un) => un()).catch(() => undefined);
      unlistenSecure.then((un) => un()).catch(() => undefined);
    };
  }, [applyCompressFolder, applyCompressSelection, loadArchive]);

  const submitSecureWizard = useCallback(async () => {
    if (!secureWizard) return;
    if (wizardPassword !== wizardConfirm) {
      setStatus("Parolalar eşleşmiyor.");
      return;
    }
    const fb = await invoke<PasswordFeedback>("evaluate_create_password", {
      password: wizardPassword,
      secureCreate: true,
    });
    if (!fb.ok_for_create) {
      setStatus(fb.hint || "Parola politikası karşılanmıyor.");
      return;
    }
    setSecureWizard(null);
    if (secureWizard.sources.length > 0) {
      await runCreateSelection(secureWizard.sources, secureWizard.suggested_output, wizardPassword);
    } else if (secureWizard.folder) {
      await runCreate(secureWizard.folder, secureWizard.suggested_output, wizardPassword);
    }
    setWizardPassword("");
    setWizardConfirm("");
  }, [runCreate, runCreateSelection, secureWizard, wizardConfirm, wizardPassword]);

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
        password: openPassword || null,
      });
      setStatus(res.message);
    } catch (e) {
      setStatus(String(e));
    } finally {
      setBusy(false);
    }
  }, [archivePath, openPassword]);

  const createZip = useCallback(async () => {
    if (encryptCreate && createPassword) {
      const fb = await invoke<PasswordFeedback>("evaluate_create_password", {
        password: createPassword,
        secureCreate: false,
      });
      if (!fb.ok_for_create) {
        setStatus(fb.hint);
        return;
      }
    }
    const source = await open({ directory: true, multiple: false });
    if (!source || Array.isArray(source)) return;
    const dest = await save({
      filters: [{ name: "ZIP", extensions: ["zip"] }],
      defaultPath: "arsiv.zip",
    });
    if (!dest) return;
    await runCreate(source, dest);
  }, [createPassword, encryptCreate, runCreate]);

  const openMigrationWizard = useCallback(async () => {
    setBusy(true);
    try {
      const guide = await invoke<WinzipMigrationGuide>("winzip_migration_guide");
      setMigrationGuide(guide);
      setMigrationOpen(true);
      if (guide.recommended_profile === "email") {
        setProfile("email");
      }
    } catch (e) {
      setStatus(String(e));
    } finally {
      setBusy(false);
    }
  }, []);

  const testZip = useCallback(async () => {
    if (!archivePath) return;
    setBusy(true);
    try {
      const res = await invoke<{ message: string }>("test_zip", {
        archive: archivePath,
        password: openPassword || null,
      });
      setStatus(res.message);
    } catch (e) {
      setStatus(String(e));
    } finally {
      setBusy(false);
    }
  }, [archivePath, openPassword]);

  return (
    <div className="app">
      <header>
        <h1>{info?.product ?? "Lertiva Zip"}</h1>
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
        <label className="password-row">
          Açma parolası
          <input
            type="password"
            value={openPassword}
            disabled={busy}
            onChange={(e) => setOpenPassword(e.target.value)}
            autoComplete="off"
          />
        </label>
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
        <label>
          <input
            type="checkbox"
            checked={encryptCreate}
            disabled={busy}
            onChange={(e) => setEncryptCreate(e.target.checked)}
          />{" "}
          AES ZIP (oluştur)
        </label>
        {encryptCreate && (
          <label className="password-row">
            Oluşturma parolası
            <input
              type="password"
              value={createPassword}
              disabled={busy}
              onChange={(e) => setCreatePassword(e.target.value)}
              autoComplete="new-password"
            />
          </label>
        )}
        <button type="button" disabled={busy} onClick={() => void createZip()}>
          Klasörden ZIP
        </button>
        <button type="button" disabled={busy} onClick={() => void openMigrationWizard()}>
          WinZip geçiş
        </button>
      </div>
      {archivePath && <p className="path">{archivePath}</p>}
      <ul className="entries">
        {entries.map((e) => (
          <li key={e}>{e}</li>
        ))}
      </ul>
      {status && <p className="status">{status}</p>}

      {migrationOpen && migrationGuide && (
        <div className="modal-overlay" role="dialog" aria-modal="true">
          <div className="modal secure-wizard migration-wizard">
            <h2>WinZip&apos;ten geçiş</h2>
            <p className="muted">
              {migrationGuide.winzip_detected
                ? `WinZip algılandı${migrationGuide.winzip_path ? `: ${migrationGuide.winzip_path}` : ""}.`
                : "WinZip kurulumu bu makinede algılanmadı — adımlar genel geçiş içindir."}
            </p>
            <p className="muted">
              Önerilen profil: <strong>{migrationGuide.recommended_profile}</strong> (e-posta eki)
            </p>
            <ol className="migration-steps">
              {migrationGuide.steps.map((step) => (
                <li key={step}>{step}</li>
              ))}
            </ol>
            <ul className="migration-notes">
              {migrationGuide.feature_notes.map((note) => (
                <li key={note}>{note}</li>
              ))}
            </ul>
            <div className="modal-actions">
              <button type="button" disabled={busy} onClick={() => setMigrationOpen(false)}>
                Kapat
              </button>
            </div>
          </div>
        </div>
      )}

      {secureWizard && (
        <div className="modal-overlay" role="dialog" aria-modal="true">
          <div className="modal secure-wizard">
            <h2>Explorer — şifreli ZIP</h2>
            <p className="muted">
              Politika: <strong>{secureWizard.policy.label}</strong> (min.{" "}
              {secureWizard.policy.min_length} karakter)
            </p>
            <p className="path wizard-path">
              Çıktı: {secureWizard.suggested_output.split(/[/\\]/).pop()}
            </p>
            <label>
              Parola
              <input
                type="password"
                value={wizardPassword}
                onChange={(e) => setWizardPassword(e.target.value)}
                autoFocus
                autoComplete="new-password"
              />
            </label>
            <label>
              Parola (tekrar)
              <input
                type="password"
                value={wizardConfirm}
                onChange={(e) => setWizardConfirm(e.target.value)}
                autoComplete="new-password"
              />
            </label>
            {wizardFeedback && (
              <p className="pw-feedback">Güç: {wizardFeedback.strength} · {wizardFeedback.hint}</p>
            )}
            <div className="modal-actions">
              <button type="button" disabled={busy} onClick={() => setSecureWizard(null)}>
                İptal
              </button>
              <button type="button" disabled={busy} onClick={() => void submitSecureWizard()}>
                Oluştur
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
