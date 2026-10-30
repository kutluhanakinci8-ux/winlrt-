//! winlrtZIP interop — I6 (W7), I4 AES (W8), I1/I2 Explorer proxy (W12), KOBİ pilot (W13), split ZIP (W20–W22), imzalı arşiv (W23).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

use winlrt_archive::profile::{
    CompressionProfile, EMAIL_ATTACHMENT_MAX_ZIP_BYTES, EMAIL_ATTACHMENT_MIXED_MAX_CREATE_MS,
};
use winlrt_archive::archive_sign;
use winlrt_archive::mobile_reader;
use winlrt_archive::secure_share;
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

#[test]
fn i3_zip64_smoke_archive_roundtrip() {
    let base = tempfile::tempdir().unwrap();
    let zip = base.path().join("z64.zip");
    zip_io::create_zip64_smoke(&zip).unwrap();
    assert!(
        zip_io::archive_indicates_zip64(&zip).unwrap(),
        "Zip64 EOCD bekleniyordu"
    );
    zip_io::test(&zip, None).unwrap();
    let out = base.path().join("out");
    zip_io::extract(&zip, &out, None).unwrap();
    assert_eq!(
        fs::read(out.join("zip64-smoke.txt")).unwrap(),
        b"zip64"
    );
}

#[test]
fn i3_zip64_list_contains_smoke_entry() {
    let base = tempfile::tempdir().unwrap();
    let zip = base.path().join("z64.zip");
    zip_io::create_zip64_smoke(&zip).unwrap();
    let entries = zip_io::list(&zip, None).unwrap();
    assert!(entries.iter().any(|e| e.contains("zip64-smoke.txt")));
}

#[test]
fn i3_7zip_opens_zip64_smoke() {
    if !seven_zip_available() {
        return;
    }
    let base = tempfile::tempdir().unwrap();
    let zip = base.path().join("z64.zip");
    zip_io::create_zip64_smoke(&zip).unwrap();
    let dest = base.path().join("7z");
    fs::create_dir_all(&dest).unwrap();
    let status = Command::new("7z")
        .args(["x", "-y", &zip.to_string_lossy(), &format!("-o{}", dest.display())])
        .status()
        .unwrap();
    assert!(status.success());
    assert_eq!(fs::read(dest.join("zip64-smoke.txt")).unwrap(), b"zip64");
}

/// Gerçek >4 GiB seyrek dosya (yerel/CI `--ignored`; disk doldurmadan Zip64 yazma).
#[test]
#[ignore = "seyrek 4GiB+ dosya — yalnizca i3_sparse test kosusunda"]
fn i3_sparse_over_4gib_zip_roundtrip() {
    use std::io::Read;
    let base = tempfile::tempdir().unwrap();
    let big = base.path().join("large.bin");
    {
        let f = fs::File::create(&big).unwrap();
        f.set_len(zip_io::ZIP64_SIZE_THRESHOLD + 1024)
            .expect("set_len (seyrek dosya desteklenmeli)");
        f.sync_all().ok();
    }
    let zip = base.path().join("large.zip");
    zip_io::create_with_profile(&big, &zip, CompressionProfile::Fast, None).unwrap();
    assert!(zip_io::archive_indicates_zip64(&zip).unwrap());
    zip_io::test(&zip, None).unwrap();
    let out = base.path().join("extract");
    zip_io::extract(&zip, &out, None).unwrap();
    let restored = out.join("large.bin");
    assert_eq!(fs::metadata(&restored).unwrap().len(), big.metadata().unwrap().len());
    let mut sample = fs::File::open(&restored).unwrap();
    let mut buf = [0u8; 16];
    sample.read_exact(&mut buf).ok();
}

const I5_TURKISH_REL: &str = "Şirket/iğüş/Özet_2026.txt";

#[test]
fn i5_turkish_utf8_filenames_roundtrip() {
    let base = tempfile::tempdir().unwrap();
    let src = base.path().join("src");
    let file = src.join(I5_TURKISH_REL);
    fs::create_dir_all(file.parent().unwrap()).unwrap();
    fs::write(&file, b"istanbul").unwrap();
    let zip = base.path().join("tr.zip");
    zip_io::create_with_profile(&src, &zip, CompressionProfile::EmailAttachment, None).unwrap();
    let entries = zip_io::list(&zip, None).unwrap();
    assert!(
        entries.iter().any(|e| e.contains("Özet_2026.txt")),
        "liste: {entries:?}"
    );
    let out = base.path().join("out");
    zip_io::extract(&zip, &out, None).unwrap();
    assert_eq!(
        fs::read(out.join(I5_TURKISH_REL)).unwrap(),
        b"istanbul"
    );
}

#[test]
fn i5_winlrt_zip_cli_list_turkish() {
    let bin = env!("CARGO_BIN_EXE_winlrt-zip");
    let base = tempfile::tempdir().unwrap();
    let src = base.path().join("src");
    let file = src.join(I5_TURKISH_REL);
    fs::create_dir_all(file.parent().unwrap()).unwrap();
    fs::write(&file, b"1").unwrap();
    let zip = base.path().join("tr-cli.zip");
    let create = Command::new(bin)
        .args([
            "create",
            src.to_str().unwrap(),
            zip.to_str().unwrap(),
            "--profile",
            "fast",
        ])
        .output()
        .unwrap();
    assert!(create.status.success());
    let list = Command::new(bin)
        .args(["list", zip.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(list.status.success());
    let out = String::from_utf8_lossy(&list.stdout);
    assert!(out.contains("Özet_2026"), "stdout: {out}");
}

#[test]
fn i5_7zip_extracts_turkish_paths() {
    if !seven_zip_available() {
        return;
    }
    let base = tempfile::tempdir().unwrap();
    let src = base.path().join("src");
    let file = src.join(I5_TURKISH_REL);
    fs::create_dir_all(file.parent().unwrap()).unwrap();
    fs::write(&file, b"7z-tr").unwrap();
    let zip = base.path().join("tr-7z.zip");
    zip_io::create_with_profile(&src, &zip, CompressionProfile::Balanced, None).unwrap();
    let dest = base.path().join("7z-out");
    fs::create_dir_all(&dest).unwrap();
    let status = Command::new("7z")
        .args(["x", "-y", &zip.to_string_lossy(), &format!("-o{}", dest.display())])
        .status()
        .unwrap();
    assert!(status.success());
    assert_eq!(fs::read(dest.join(I5_TURKISH_REL)).unwrap(), b"7z-tr");
}

fn assert_crc_error(err: impl std::fmt::Display) {
    let s = err.to_string();
    assert!(
        s.contains(winlrt_archive::zip_io::ZIP_INTEGRITY_CRC_MSG),
        "I7 mesaji bekleniyordu, alinan: {s}"
    );
}

#[test]
fn i7_corrupt_zip_test_fails_controlled_message() {
    let base = tempfile::tempdir().unwrap();
    let src = base.path().join("veri.txt");
    fs::write(&src, b"winlrtZIP I7").unwrap();
    let zip = base.path().join("bad.zip");
    zip_io::create_with_profile(&src, &zip, CompressionProfile::Balanced, None).unwrap();
    zip_io::corrupt_first_local_crc_for_test(&zip).unwrap();
    assert_crc_error(zip_io::test(&zip, None).unwrap_err());
}

#[test]
fn i7_corrupt_zip_extract_fails_controlled_message() {
    let base = tempfile::tempdir().unwrap();
    let src = base.path().join("veri.txt");
    fs::write(&src, b"winlrtZIP I7 extract").unwrap();
    let zip = base.path().join("bad.zip");
    zip_io::create_with_profile(&src, &zip, CompressionProfile::Balanced, None).unwrap();
    zip_io::corrupt_first_local_crc_for_test(&zip).unwrap();
    let out = base.path().join("out");
    assert_crc_error(zip_io::extract(&zip, &out, None).unwrap_err());
}

#[test]
fn i7_winlrt_zip_cli_test_corrupt() {
    let bin = env!("CARGO_BIN_EXE_winlrt-zip");
    let base = tempfile::tempdir().unwrap();
    let src = base.path().join("x.txt");
    fs::write(&src, b"cli i7").unwrap();
    let zip = base.path().join("bad-cli.zip");
    let create = Command::new(bin)
        .args([
            "create",
            src.to_str().unwrap(),
            zip.to_str().unwrap(),
            "--profile",
            "balanced",
        ])
        .output()
        .unwrap();
    assert!(create.status.success());
    zip_io::corrupt_first_local_crc_for_test(&zip).unwrap();
    let test = Command::new(bin)
        .args(["test", zip.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(!test.status.success());
    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&test.stderr),
        String::from_utf8_lossy(&test.stdout)
    );
    assert!(
        combined.contains("CRC") || combined.contains("bozuk"),
        "cli cikti: {combined}"
    );
}

fn info_zip_available() -> bool {
    Command::new("zip")
        .arg("-h")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn info_unzip_available() -> bool {
    Command::new("unzip")
        .arg("-h")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Explorer “Sıkıştırılmış klasör” benzeri: Info-ZIP ile oluştur → winlrt aç (I1 proxy).
#[test]
fn i1_infozip_archive_winlrt_test_and_extract() {
    if !info_zip_available() {
        eprintln!("zip yok — I1 proxy atlandi");
        return;
    }
    let base = tempfile::tempdir().unwrap();
    let src = base.path().join("src");
    let nested = src.join("Belgeler").join("not.txt");
    fs::create_dir_all(nested.parent().unwrap()).unwrap();
    fs::write(&nested, b"explorer-i1-proxy").unwrap();
    fs::write(src.join("ust.txt"), b"ust").unwrap();
    let zip = base.path().join("from-infozip.zip");
    let status = Command::new("zip")
        .args([
            "-r",
            "-q",
            zip.to_str().unwrap(),
            "Belgeler/not.txt",
            "ust.txt",
        ])
        .current_dir(&src)
        .status()
        .expect("zip calistir");
    assert!(status.success(), "Info-ZIP arsiv olusturamadi");
    zip_io::test(&zip, None).unwrap();
    let out = base.path().join("winlrt-out");
    zip_io::extract(&zip, &out, None).unwrap();
    assert_eq!(
        fs::read(out.join("Belgeler/not.txt")).unwrap(),
        b"explorer-i1-proxy"
    );
    assert_eq!(fs::read(out.join("ust.txt")).unwrap(), b"ust");
}

#[test]
fn i1_winlrt_zip_cli_opens_infozip_archive() {
    if !info_zip_available() {
        return;
    }
    let bin = env!("CARGO_BIN_EXE_winlrt-zip");
    let base = tempfile::tempdir().unwrap();
    let src = base.path().join("a.txt");
    fs::write(&src, b"i1-cli").unwrap();
    let zip = base.path().join("foreign.zip");
    let status = Command::new("zip")
        .args(["-q", zip.file_name().unwrap().to_str().unwrap(), "a.txt"])
        .current_dir(base.path())
        .status()
        .unwrap();
    assert!(status.success());
    let test = Command::new(bin)
        .args(["test", zip.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(
        test.status.success(),
        "test: {}",
        String::from_utf8_lossy(&test.stderr)
    );
}

/// winlrt ZIP → harici arac cikarir (Explorer “Tumunu cikar” proxy: Info-ZIP unzip).
#[test]
fn i2_winlrt_zip_unzip_extracts_without_error() {
    if !info_unzip_available() {
        eprintln!("unzip yok — I2 proxy atlandi");
        return;
    }
    let base = tempfile::tempdir().unwrap();
    let src = base.path().join("src");
    fs::create_dir_all(&src).unwrap();
    fs::write(src.join("rapor.txt"), b"winlrt-i2").unwrap();
    let zip = base.path().join("winlrt-made.zip");
    zip_io::create_with_profile(&src, &zip, CompressionProfile::Balanced, None).unwrap();
    let dest = base.path().join("unzip-out");
    fs::create_dir_all(&dest).unwrap();
    let status = Command::new("unzip")
        .args(["-o", "-q", zip.to_str().unwrap()])
        .current_dir(&dest)
        .status()
        .unwrap();
    assert!(status.success(), "unzip winlrt ZIP acamadi");
    assert_eq!(
        fs::read(dest.join("src/rapor.txt")).unwrap_or_else(|_| {
            fs::read(dest.join("rapor.txt")).unwrap()
        }),
        b"winlrt-i2"
    );
}

#[test]
fn i2_7zip_extracts_winlrt_balanced_zip() {
    if !seven_zip_available() {
        return;
    }
    let base = tempfile::tempdir().unwrap();
    let src = base.path().join("x.txt");
    fs::write(&src, b"7z-i2").unwrap();
    let zip = base.path().join("w.zip");
    zip_io::create_with_profile(&src, &zip, CompressionProfile::EmailAttachment, None).unwrap();
    let dest = base.path().join("7z");
    fs::create_dir_all(&dest).unwrap();
    let status = Command::new("7z")
        .args(["x", "-y", &zip.to_string_lossy(), &format!("-o{}", dest.display())])
        .status()
        .unwrap();
    assert!(status.success());
    assert_eq!(fs::read(dest.join("x.txt")).unwrap(), b"7z-i2");
}

#[test]
fn i2_winlrt_zip_cli_create_unzip_roundtrip() {
    if !info_unzip_available() {
        return;
    }
    let bin = env!("CARGO_BIN_EXE_winlrt-zip");
    let base = tempfile::tempdir().unwrap();
    let src = base.path().join("src");
    fs::create_dir_all(&src).unwrap();
    fs::write(src.join("i2.txt"), b"cli-i2").unwrap();
    let zip = base.path().join("cli.zip");
    let create = Command::new(bin)
        .args([
            "create",
            src.to_str().unwrap(),
            zip.to_str().unwrap(),
            "--profile",
            "balanced",
        ])
        .output()
        .unwrap();
    assert!(create.status.success());
    let dest = base.path().join("out");
    fs::create_dir_all(&dest).unwrap();
    let status = Command::new("unzip")
        .args(["-o", "-q", zip.to_str().unwrap()])
        .current_dir(&dest)
        .status()
        .unwrap();
    assert!(status.success());
    let content = fs::read(dest.join("src/i2.txt"))
        .or_else(|_| fs::read(dest.join("i2.txt")))
        .unwrap();
    assert_eq!(content, b"cli-i2");
}

/// KOBİ: fatura/evrak klasörü → e-posta profili ZIP (≤10 MB) → alıcı test/çıkar (W13 proxy).
#[test]
fn kobi_invoice_bundle_email_profile_under_cap() {
    let base = tempfile::tempdir().unwrap();
    let src = base.path().join("FaturaPaketi");
    fs::create_dir_all(&src).unwrap();
    for i in 0..4 {
        fs::write(
            src.join(format!("fatura_{i:03}.pdf.txt")),
            vec![b'F'; 512 * 1024],
        )
        .unwrap();
    }
    fs::write(src.join("ozet.txt"), b"KOBI W13 pilot").unwrap();
    let zip = base.path().join("mail-eki.zip");
    zip_io::create_with_profile(&src, &zip, CompressionProfile::EmailAttachment, None).unwrap();
    let size = fs::metadata(&zip).unwrap().len();
    assert!(
        CompressionProfile::email_attachment_size_ok(size),
        "KOBİ ZIP {size} B > {EMAIL_ATTACHMENT_MAX_ZIP_BYTES} B"
    );
    zip_io::test(&zip, None).unwrap();
    let out = base.path().join("alici");
    zip_io::extract(&zip, &out, None).unwrap();
    assert_eq!(
        fs::read(out.join("FaturaPaketi/ozet.txt")).unwrap_or_else(|_| {
            fs::read(out.join("ozet.txt")).unwrap()
        }),
        b"KOBI W13 pilot"
    );
}

#[test]
fn kobi_winlrt_zip_cli_email_workflow() {
    let bin = env!("CARGO_BIN_EXE_winlrt-zip");
    let base = tempfile::tempdir().unwrap();
    let src = base.path().join("Evrak");
    fs::create_dir_all(&src).unwrap();
    fs::write(src.join("teklif.txt"), vec![b'T'; 256 * 1024]).unwrap();
    let zip = base.path().join("gonder.zip");
    let create = Command::new(bin)
        .args([
            "create",
            src.to_str().unwrap(),
            zip.to_str().unwrap(),
            "--profile",
            "email",
        ])
        .output()
        .unwrap();
    assert!(
        create.status.success(),
        "create: {}",
        String::from_utf8_lossy(&create.stderr)
    );
    let size = fs::metadata(&zip).unwrap().len();
    assert!(CompressionProfile::email_attachment_size_ok(size));
    let test = Command::new(bin)
        .args(["test", zip.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(test.status.success());
    let dest = base.path().join("inbox");
    fs::create_dir_all(&dest).unwrap();
    let extract = Command::new(bin)
        .args([
            "extract",
            zip.to_str().unwrap(),
            dest.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(extract.status.success());
    let body = fs::read(dest.join("Evrak/teklif.txt"))
        .or_else(|_| fs::read(dest.join("teklif.txt")))
        .unwrap();
    assert_eq!(body.len(), 256 * 1024);
}

#[test]
fn w20_split_zip_eocd_smoke_detected() {
    let base = tempfile::tempdir().unwrap();
    let src = base.path().join("vol.txt");
    fs::write(&src, b"w20-split").unwrap();
    let zip = base.path().join("w20.zip");
    zip_io::create_with_profile(&src, &zip, CompressionProfile::Balanced, None).unwrap();
    zip_io::mark_split_zip_eocd_smoke_for_test(&zip).unwrap();
    assert!(zip_io::archive_indicates_split_zip(&zip).unwrap());
}

#[test]
fn w20_split_zip_companion_z01_on_disk() {
    let base = tempfile::tempdir().unwrap();
    let zip = base.path().join("set.zip");
    fs::write(&zip, b"PK\x03\x04").unwrap();
    fs::write(base.path().join("set.z01"), b"part2").unwrap();
    let vols = zip_io::split_zip_volumes_on_disk(&zip);
    assert!(vols.len() >= 2);
    assert!(zip_io::archive_indicates_split_zip(&zip).unwrap());
}

#[test]
fn w20_normal_zip_not_split() {
    let base = tempfile::tempdir().unwrap();
    let src = base.path().join("n.txt");
    fs::write(&src, b"1").unwrap();
    let zip = base.path().join("n.zip");
    zip_io::create_with_profile(&src, &zip, CompressionProfile::Fast, None).unwrap();
    assert!(!zip_io::archive_indicates_split_zip(&zip).unwrap());
}

#[test]
fn w22_split_zip_create_and_extract() {
    let base = tempfile::tempdir().unwrap();
    fs::write(base.path().join("w22.txt"), b"w22-create-split").unwrap();
    let dest = base.path().join("pkg.zip");
    let info = zip_io::create_split_with_profile(
        &base.path().join("w22.txt"),
        &dest,
        CompressionProfile::Fast,
        None,
        zip_io::SPLIT_ZIP_MIN_VOLUME_BYTES,
    )
    .unwrap();
    assert_eq!(info.volume_count, 1);
    zip_io::test(&dest, None).unwrap();
    let out = base.path().join("out");
    zip_io::extract(&dest, &out, None).unwrap();
    assert_eq!(fs::read(out.join("w22.txt")).unwrap(), b"w22-create-split");

    let big = base.path().join("large.bin");
    fs::write(&big, vec![1u8; 140 * 1024]).unwrap();
    let dest2 = base.path().join("big.zip");
    let info2 = zip_io::create_split_with_profile(
        &big,
        &dest2,
        CompressionProfile::Fast,
        None,
        zip_io::SPLIT_ZIP_MIN_VOLUME_BYTES,
    )
    .unwrap();
    assert!(info2.volume_count >= 2);
    zip_io::extract(&dest2, &base.path().join("out2"), None).unwrap();
    assert_eq!(fs::read(base.path().join("out2/large.bin")).unwrap().len(), 140 * 1024);
}

#[test]
fn w21_split_zip_synthetic_merge_extract() {
    let base = tempfile::tempdir().unwrap();
    let src = base.path().join("w21.txt");
    fs::write(&src, b"w21-interop-merge").unwrap();
    let whole = base.path().join("full.zip");
    zip_io::create_with_profile(&src, &whole, CompressionProfile::Fast, None).unwrap();
    let bytes = fs::read(&whole).unwrap();
    let mid = bytes.len() / 2;
    fs::write(base.path().join("w21.z01"), &bytes[..mid]).unwrap();
    fs::write(base.path().join("w21.zip"), &bytes[mid..]).unwrap();
    assert!(zip_io::archive_indicates_split_zip(&base.path().join("w21.zip")).unwrap());
    zip_io::test(&base.path().join("w21.zip"), None).unwrap();
    let out = base.path().join("out");
    zip_io::extract(&base.path().join("w21.z01"), &out, None).unwrap();
    assert_eq!(fs::read(out.join("w21.txt")).unwrap(), b"w21-interop-merge");
}

#[test]
fn w23_signed_archive_extract() {
    let base = tempfile::tempdir().unwrap();
    let src = base.path().join("signed.txt");
    fs::write(&src, b"w23-ed25519-sidecar").unwrap();
    let zip = base.path().join("corp.zip");
    zip_io::create_with_profile(&src, &zip, CompressionProfile::Fast, None).unwrap();
    let key = archive_sign::signing_key_from_seed([0x23u8; 32]);
    zip_io::sign_zip_archive(&zip, &key).unwrap();
    zip_io::test(&zip, None).unwrap();
    let out = base.path().join("out");
    zip_io::extract(&zip, &out, None).unwrap();
    assert_eq!(fs::read(out.join("signed.txt")).unwrap(), b"w23-ed25519-sidecar");
}

#[test]
fn w28_lerta_mail_attachment_webhook() {
    let base = tempfile::tempdir().unwrap();
    let src = base.path().join("lerta.txt");
    fs::write(&src, b"w28-lerta-mail").unwrap();
    let zip = base.path().join("posta.zip");
    let prep = winlrt_archive::lerta_mail::prepare_lerta_mail_attachment(
        &src,
        &zip,
        winlrt_archive::lerta_mail::LertaAttachmentTier::Standard10Mb,
        None,
    )
    .unwrap();
    let hook = winlrt_archive::lerta_mail::webhook_payload_from_prepare(&prep);
    assert!(hook.kvkk_local_only);
    assert_eq!(hook.mime, "application/zip");
    winlrt_archive::lerta_mail::validate_lerta_mail_attachment(
        &zip,
        winlrt_archive::lerta_mail::LertaAttachmentTier::Standard10Mb,
    )
    .unwrap();
}

#[test]
fn w27_store_gate_all_pass_in_repo() {
    let root = winlrt_archive::store_gate::default_workspace_root();
    let report = winlrt_archive::store_gate::evaluate_store_gate(&root).unwrap();
    assert!(report.all_pass, "store gate: {:?}", report.items);
    assert_eq!(report.bundle_identifier.as_deref(), Some("com.winlrt.zip"));
}

#[test]
fn w26_mobile_inspect_read_only_capabilities() {
    let caps = mobile_reader::capabilities();
    assert!(!caps.create_archive);
    assert!(caps.lerta_share_intent_ready);
    let base = tempfile::tempdir().unwrap();
    let src = base.path().join("mob.txt");
    fs::write(&src, b"w26-mobile").unwrap();
    let zip = base.path().join("mob.zip");
    zip_io::create_with_profile(&src, &zip, CompressionProfile::Fast, None).unwrap();
    let report = mobile_reader::inspect(&zip, None).unwrap();
    assert!(report.read_only);
    assert!(!report.entries_preview.is_empty());
}

#[test]
fn w25_secure_share_pack_kvkk_local() {
    let base = tempfile::tempdir().unwrap();
    let src = base.path().join("brief.txt");
    fs::write(&src, b"w25-kvkk-local-pack").unwrap();
    let zip = base.path().join("share.zip");
    let pw = "Share-Pack12";
    let pack = secure_share::create_secure_share_pack(
        &src,
        &zip,
        pw,
        Some("https://intranet.example"),
        Some(14),
    )
    .unwrap();
    assert!(pack.manifest_path.is_file());
    let manifest = secure_share::verify_secure_share_pack(&zip, Some(pw)).unwrap();
    assert!(manifest.kvkk_local_only);
    let link = secure_share::self_hosted_download_url(&manifest, "https://intranet.example");
    assert!(link.starts_with("https://intranet.example/winlrt-share/"));
    zip_io::extract(&zip, &base.path().join("out"), Some(pw)).unwrap();
    assert_eq!(fs::read(base.path().join("out/brief.txt")).unwrap(), b"w25-kvkk-local-pack");
}

#[test]
fn w24_infozip_split_span_open_extract() {
    if Command::new("zip").arg("-h").output().is_err() {
        return;
    }
    let base = tempfile::tempdir().unwrap();
    fs::write(base.path().join("w24.bin"), vec![0x24u8; 180 * 1024]).unwrap();
    let status = Command::new("zip")
        .current_dir(base.path())
        .args(["-0", "-s", "64k", "w24.zip", "w24.bin"])
        .status()
        .unwrap();
    if !status.success() {
        return;
    }
    let vols = zip_io::split_zip_volumes_on_disk(&base.path().join("w24.zip"));
    assert!(zip_io::volume_has_infozip_span_marker(&vols[0]).unwrap());
    zip_io::test(&base.path().join("w24.zip"), None).unwrap();
    zip_io::extract(&base.path().join("w24.z01"), &base.path().join("out"), None).unwrap();
    assert_eq!(fs::read(base.path().join("out/w24.bin")).unwrap().len(), 180 * 1024);
}

#[test]
fn w23_require_signed_archive_policy_blocks_unsigned() {
    let base = tempfile::tempdir().unwrap();
    let src = base.path().join("u.txt");
    fs::write(&src, b"unsigned").unwrap();
    let zip = base.path().join("unsigned.zip");
    zip_io::create_with_profile(&src, &zip, CompressionProfile::Fast, None).unwrap();
    std::env::set_var("WINLRT_REQUIRE_SIGNED_ARCHIVE", "1");
    let err = zip_io::extract(&zip, &base.path().join("out"), None).unwrap_err();
    std::env::remove_var("WINLRT_REQUIRE_SIGNED_ARCHIVE");
    assert!(err.to_string().contains("eksik"));
}
