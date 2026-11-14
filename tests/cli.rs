use std::path::Path;

use assert_cmd::{Command, cargo::cargo_bin_cmd};

fn items(data_dir: &Path) -> Command {
    let mut command = cargo_bin_cmd!("items");
    command.arg("--data-dir").arg(data_dir);
    command
}

#[test]
fn notebook_lifecycle_across_processes() {
    let dir = tempfile::tempdir().unwrap();
    items(dir.path())
        .arg("list")
        .assert()
        .success()
        .stdout("No items yet. Use 'items add <title>' to save one.\n");
    items(dir.path())
        .args(["add", "Shopping", "--body", "Milk\nBread"])
        .assert()
        .success()
        .stdout("Added item 1.\n");
    items(dir.path())
        .arg("list")
        .assert()
        .success()
        .stdout("1  Shopping\n");
    items(dir.path())
        .args(["show", "1"])
        .assert()
        .success()
        .stdout("1  Shopping\n\nMilk\nBread\n");
    items(dir.path())
        .args(["remove", "1"])
        .assert()
        .success()
        .stdout("Removed item 1.\n");
    items(dir.path()).args(["show", "1"]).assert().failure();
    items(dir.path())
        .args(["add", "Another"])
        .assert()
        .success()
        .stdout("Added item 2.\n");
}

#[test]
fn bad_input_does_not_create_data() {
    let dir = tempfile::tempdir().unwrap();
    for args in [vec!["add", "  "], vec!["show", "abc"], vec!["remove", "99"]] {
        items(dir.path()).args(args).assert().failure();
    }
    assert!(!dir.path().join("items.json").exists());
}

#[test]
fn corrupt_file_is_not_overwritten() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("items.json");
    std::fs::write(&path, "not json").unwrap();
    let output = items(dir.path()).args(["add", "New"]).assert().failure();
    assert!(String::from_utf8_lossy(&output.get_output().stderr).contains("Invalid JSON"));
    assert_eq!(std::fs::read_to_string(path).unwrap(), "not json");
}
