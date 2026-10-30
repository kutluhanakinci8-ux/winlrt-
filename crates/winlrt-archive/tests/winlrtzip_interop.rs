//! winlrtZIP interop — I6 (W7), I4 AES (W8).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

use winlrt_archive::profile::{
    CompressionProfile, EMAIL_ATTACHMENT_MAX_ZIP_BYTES, EMAIL_ATTACHMENT_MIXED_MAX_CREATE_MS,
};
use winlrt_archive::zip_io;

/// Kurumsal politika ile uyumlu test parolası.
const I4_AES_PASSWORD: &str = "Winlrt-Pilot12";

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn i6_email_profile_small_fixture_under_10mb() {
    let base = tempfile::tempdir().unwrap();
    let src = base.path().join("src");
    fs::create_dir_all(&src).unwrap();
    for i in 0..6 {
        fs::write(src.join(format!("part{i}.txt")), vec![b'a'; 1024 * 1024]).unwrap();
    }
    let zip = base.path().join("mail.zip");
    let t0 = Instant::now();
    zip_io::create_with_profile(&src, &zip, CompressionProfile::EmailAttachment, None).unwrap();
    let ms = t0.elapsed().as_millis();
    let size = fs::metadata(&zip).unwrap().len();
    assert!(
        CompressionProfile::email_attachment_size_ok(size),
        "ZIP {size} B > {EMAIL_ATTACHMENT_MAX_ZIP_BYTES} B (email profili, {ms} ms)"
    );
}

#[test]
#[ignore = "benchmarks/fixtures/mixed/prepare.sh gerekir"]
fn i6_email_profile_mixed_fixture_gate() {
    let data = workspace_root().join("benchmarks/fixtures/mixed/data");
    assert!(
        data.is_dir(),
        "mixed/data yok — bash benchmarks/fixtures/mixed/prepare.sh"
    );
    let tmp = tempfile::tempdir().unwrap();
    let out = tmp.path().join("email-mixed.zip");
    let t0 = Instant::now();
    zip_io::create_with_profile(&data, &out, CompressionProfile::EmailAttachment, None).unwrap();
    let ms = t0.elapsed().as_millis();
    let size = fs::metadata(&out).unwrap().len();
    assert!(
        CompressionProfile::email_attachment_size_ok(size),
        "mixed email ZIP {size} B > cap {EMAIL_ATTACHMENT_MAX_ZIP_BYTES} B"
    );
    assert!(
        ms <= EMAIL_ATTACHMENT_MIXED_MAX_CREATE_MS,
        "mixed email create {ms} ms > {EMAIL_ATTACHMENT_MIXED_MAX_CREATE_MS} ms"
    );
    eprintln!("I6 mixed: {size} B, {ms} ms");
}

/// WinZip AE-2: central directory AES extra field (0x9901).
fn assert_winzip_aes_extra_field(archive: &Path) {
    let bytes = fs::read(archive).expect("zip okunamadi");
    assert!(
        bytes.windows(2).any(|w| w == [0x01, 0x99]),
        "WinZip AES extra field (0x9901) bulunamadi"
    );
}

#[test]
fn i4_aes256_roundtrip_and_winzip_marker() {
    let base = tempfile::tempdir().unwrap();
    let src = base.path().join("src");
    fs::create_dir_all(&src).unwrap();
    fs::write(src.join("gizli.txt"), b"winlrtZIP I4 AES").unwrap();
    let zip = base.path().join("aes.zip");
    zip_io::create_with_profile(
        &src,
        &zip,
        CompressionProfile::EmailAttachment,
        Some(I4_AES_PASSWORD),
    )
    .unwrap();
    assert_winzip_aes_extra_field(&zip);
    zip_io::test(&zip, Some(I4_AES_PASSWORD)).unwrap();
    let out = base.path().join("out");
    zip_io::extract(&zip, &out, Some(I4_AES_PASSWORD)).unwrap();
    assert_eq!(
        fs::read(out.join("gizli.txt")).unwrap(),
        b"winlrtZIP I4 AES"
    );
}

#[test]
fn i4_aes_wrong_password_rejected() {
    let base = tempfile::tempdir().unwrap();
    let src = base.path().join("a.txt");
    fs::write(&src, b"x").unwrap();
    let zip = base.path().join("enc.zip");
    zip_io::create_with_profile(
        &src,
        &zip,
        CompressionProfile::Balanced,
        Some(I4_AES_PASSWORD),
    )
    .unwrap();
    assert!(zip_io::extract(&zip, base.path(), Some("yanlis-parola")).is_err());
}

fn seven_zip_available() -> bool {
    Command::new("7z")
        .arg("--help")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

#[test]
fn i4_7zip_opens_winlrt_aes_zip() {
    if !seven_zip_available() {
        eprintln!("7z yok — I4 dis arac testi atlandi");
        return;
    }
    let base = tempfile::tempdir().unwrap();
    let src = base.path().join("src");
    fs::create_dir_all(&src).unwrap();
    fs::write(src.join("interop.txt"), b"7zip+winlrt").unwrap();
    let zip = base.path().join("aes-7z.zip");
    zip_io::create_with_profile(
        &src,
        &zip,
        CompressionProfile::Balanced,
        Some(I4_AES_PASSWORD),
    )
    .unwrap();
    let dest = base.path().join("7z-out");
    fs::create_dir_all(&dest).unwrap();
    let status = Command::new("7z")
        .args([
            "x",
            "-y",
            &format!("-p{I4_AES_PASSWORD}"),
            &zip.to_string_lossy(),
            &format!("-o{}", dest.display()),
        ])
        .status()
        .expect("7z calistir");
    assert!(status.success(), "7-Zip AES ZIP acamadi");
    assert_eq!(
        fs::read(dest.join("interop.txt")).unwrap_or_default(),
        b"7zip+winlrt"
    );
}

#[test]
fn i4_winlrt_zip_cli_aes_roundtrip() {
    let bin = env!("CARGO_BIN_EXE_winlrt-zip");
    let base = tempfile::tempdir().unwrap();
    let src = base.path().join("src");
    fs::create_dir_all(&src).unwrap();
    fs::write(src.join("cli.txt"), b"cli aes").unwrap();
    let zip = base.path().join("cli-aes.zip");
    let create = Command::new(bin)
        .args([
            "create",
            src.to_str().unwrap(),
            zip.to_str().unwrap(),
            "--profile",
            "email",
            "--password",
            I4_AES_PASSWORD,
        ])
        .output()
        .unwrap();
    assert!(
        create.status.success(),
        "create: {}",
        String::from_utf8_lossy(&create.stderr)
    );
    assert_winzip_aes_extra_field(&zip);
    let test = Command::new(bin)
        .args([
            "test",
            zip.to_str().unwrap(),
            "--password",
            I4_AES_PASSWORD,
        ])
        .output()
        .unwrap();
    assert!(test.status.success());
}
