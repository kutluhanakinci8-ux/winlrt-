use std::path::PathBuf;

use serde::Serialize;

#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
pub struct WinzipMigrationGuide {
    pub winzip_detected: bool,
    pub winzip_path: Option<String>,
    pub recommended_profile: &'static str,
    pub steps: Vec<String>,
    pub feature_notes: Vec<String>,
}

#[cfg(windows)]
fn winzip_install_candidates() -> [PathBuf; 4] {
    [
        PathBuf::from(r"C:\Program Files\WinZip\WINZIP32.EXE"),
        PathBuf::from(r"C:\Program Files\WinZip\WinZip.exe"),
        PathBuf::from(r"C:\Program Files (x86)\WinZip\WINZIP32.EXE"),
        PathBuf::from(r"C:\Program Files (x86)\WinZip\WinZip.exe"),
    ]
}

#[cfg(windows)]
fn detect_winzip() -> Option<PathBuf> {
    winzip_install_candidates()
        .into_iter()
        .find(|p| p.is_file())
}

#[cfg(not(windows))]
fn detect_winzip() -> Option<PathBuf> {
    None
}

pub fn winzip_migration_guide() -> WinzipMigrationGuide {
    let winzip_path = detect_winzip().map(|p| p.to_string_lossy().into_owned());
    let winzip_detected = winzip_path.is_some();

    let mut steps = vec![
        "Mevcut WinZip .zip arsivlerinizi Lertiva Zip ile acin (AES ve klasik ZIP).".into(),
        "Yeni sifreli arsivlerde AES-256 secin; ZipCrypto kullanilmaz.".into(),
        "E-posta eki icin profil: E-posta eki (~10 MB hedef).".into(),
    ];
    if cfg!(target_os = "windows") {
        steps.push("Explorer: Lertiva Zip — hizli ZIP veya sag tik ile sikistir.".into());
    } else if cfg!(target_os = "macos") {
        steps.push("macOS: .zip dosyasini Lertiva Zip penceresine surukleyin.".into());
    }
    if winzip_detected {
        steps.insert(
            0,
            "WinZip kurulumu algilandi — gecis sirasinda iki uygulama yan yana calisabilir.".into(),
        );
    }

    let feature_notes = vec![
        "WinZip Share (bulut): Lertiva Zip yerel + e-posta profili (Faz 5 opsiyonel paylasim).".into(),
        "Abonelik yok: tek kurulum + kurumsal lisans modeli.".into(),
        "AES-256 WinZip uyumlu (I4 interop testleri).".into(),
    ];

    WinzipMigrationGuide {
        winzip_detected,
        winzip_path,
        recommended_profile: "email",
        steps,
        feature_notes,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn migration_guide_has_steps_and_profile() {
        let g = winzip_migration_guide();
        assert!(!g.steps.is_empty());
        assert!(!g.feature_notes.is_empty());
        assert_eq!(g.recommended_profile, "email");
        #[cfg(not(windows))]
        assert!(!g.winzip_detected);
    }
}
