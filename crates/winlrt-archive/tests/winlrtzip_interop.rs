//! winlrtZIP interop — I6 e-posta profili (W7).

use std::fs;
use std::path::PathBuf;
use std::time::Instant;

use winlrt_archive::profile::{CompressionProfile, EMAIL_ATTACHMENT_MAX_ZIP_BYTES, EMAIL_ATTACHMENT_MIXED_MAX_CREATE_MS};
use winlrt_archive::zip_io;

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
