use std::path::Path;

use assert_cmd::{Command, cargo::cargo_bin_cmd};

fn items(data_dir: &Path) -> Command {
    let mut command = cargo_bin_cmd!("items");
    command.arg("--data-dir").arg(data_dir);
    command
}

#[test]
fn json_list_contains_saved_items() {
    let dir = tempfile::tempdir().unwrap();
    items(dir.path())
        .args(["add", "Say \"hi\"", "--body", "Milk\n사과"])
        .assert()
        .code(0);
    items(dir.path()).args(["add", "Next"]).assert().code(0);
    let output = items(dir.path())
        .args(["list", "--json"])
        .assert()
        .code(0)
        .stderr("");
    let stdout = &output.get_output().stdout;
    let value: serde_json::Value = serde_json::from_slice(stdout).unwrap();
    assert_eq!(
        value,
        serde_json::json!([
            {"id": 1, "title": "Say \"hi\"", "body": "Milk\n사과"},
            {"id": 2, "title": "Next", "body": ""}
        ])
    );
    assert!(stdout.ends_with(b"\n"));
    assert_eq!(stdout.iter().filter(|&&byte| byte == b'\n').count(), 1);
}

#[test]
fn json_list_is_empty() {
    let dir = tempfile::tempdir().unwrap();
    items(dir.path())
        .args(["list", "--json"])
        .assert()
        .code(0)
        .stdout("[]\n")
        .stderr("");
}

#[test]
fn list_rejects_unknown_options() {
    let dir = tempfile::tempdir().unwrap();
    let output = items(dir.path())
        .args(["list", "--bogus"])
        .assert()
        .code(2)
        .stdout("");
    let stderr = String::from_utf8_lossy(&output.get_output().stderr);
    assert!(stderr.contains("unexpected argument '--bogus'"));
    assert!(stderr.contains("--help"));
}

#[test]
fn json_list_reports_corrupt_data() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("items.json"), "not json").unwrap();
    let output = items(dir.path())
        .args(["list", "--json"])
        .assert()
        .code(1)
        .stdout("");
    assert!(String::from_utf8_lossy(&output.get_output().stderr).contains("Invalid JSON"));
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
