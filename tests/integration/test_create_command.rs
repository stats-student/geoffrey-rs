use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::path::Path;
use std::process::Command;
use test_fixtures::test_in_tmp_dir;


#[test]
fn creates_new_project() -> () {
    test_in_tmp_dir(|| {
        let mut cmd = Command::cargo_bin("geoff").unwrap();

        cmd.arg("create").arg("test_project");
        cmd.assert().success();

        assert!(Path::new("./test_project/").exists())
    })
}

#[test]
fn create_prints_created_message() -> () {
    test_in_tmp_dir(|| {
        let mut cmd = Command::cargo_bin("geoff").unwrap();

        cmd.arg("create").arg("test_project");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("\u{1F680} test_project created!"));

        ()
    })
}
