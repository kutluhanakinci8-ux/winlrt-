use std::fs;
use std::process::Command;

use tempfile::tempdir;

#[test]
fn winlrt_zip_create_list_test() {
    let bin = env!("CARGO_BIN_EXE_lertiva-zip");
    let work = tempdir().unwrap();
    let src = work.path().join("src");
    fs::create_dir_all(&src).unwrap();
    fs::write(src.join("hello.txt"), b"Lertiva Zip smoke").unwrap();
    let zip_path = work.path().join("out.zip");

    let create = Command::new(bin)
        .args([
            "create",
            src.to_str().unwrap(),
            zip_path.to_str().unwrap(),
            "--profile",
            "fast",
        ])
        .output()
        .unwrap();
    assert!(
        create.status.success(),
        "create: {}",
        String::from_utf8_lossy(&create.stderr)
    );

    let list = Command::new(bin)
        .args(["list", zip_path.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(list.status.success());
    let out = String::from_utf8_lossy(&list.stdout);
    assert!(out.contains("hello.txt"));

    let test = Command::new(bin)
        .args(["test", zip_path.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(test.status.success());
}
