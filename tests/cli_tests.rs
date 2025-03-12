use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;

#[test]
fn test_create_project() {
    let dir = tempdir().unwrap();
    let project_path = dir.path().join("test_project");

    let mut cmd = Command::cargo_bin("iso-env").unwrap();
    cmd.current_dir(&dir)
        .arg("create")
        .arg("test_project")
        .arg("rust")
        .arg("1.0");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Project test_project created with rust version 1.0!"));

    assert!(project_path.exists());
}

#[test]
fn test_list_projects() {
    let dir = tempdir().unwrap();

    let mut cmd = Command::cargo_bin("iso-env").unwrap();
    cmd.current_dir(&dir)
        .arg("list");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Managed Projects:"));
}

#[test]
fn test_switch_environment() {
    let dir = tempdir().unwrap();

    // Create a project
    let mut create_cmd = Command::cargo_bin("iso-env").unwrap();
    create_cmd.current_dir(&dir)
        .arg("create")
        .arg("test_project")
        .arg("rust")
        .arg("1.0");
    create_cmd.assert()
        .success()
        .stdout(predicate::str::contains("Project test_project created with rust version 1.0!"));

    // Switch to the project
    let mut cmd = Command::cargo_bin("iso-env").unwrap();
    cmd.current_dir(&dir)
        .arg("use")
        .arg("test_project");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Switched to environment test_project"));
}