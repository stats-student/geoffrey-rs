use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::{env, fs, path, process};
use test_fixtures::test_in_tmp_dir;

#[test]
fn creates_new_project() -> () {
    test_in_tmp_dir(
        || {
            let mut cmd: process::Command = process::Command::cargo_bin("geoff").unwrap();

            cmd.arg("create").arg("test_project");
            cmd.assert().success();

            assert!(path::Path::new("./test_project/").exists())
        },
        false,
    )
}

#[test]
fn create_prints_created_message() -> () {
    test_in_tmp_dir(
        || {
            let mut cmd: process::Command = process::Command::cargo_bin("geoff").unwrap();

            cmd.arg("create").arg("test_project");
            cmd.assert()
                .success()
                .stdout(predicate::str::contains("\u{1F680} test_project created!"));

            ()
        },
        false,
    )
}

#[test]
fn creates_new_project_path_passed() -> () {
    test_in_tmp_dir(
        || {
            let mut cmd: process::Command = process::Command::cargo_bin("geoff").unwrap();

            cmd.arg("create")
                .arg("--parents")
                .arg("path/to/test_project");
            cmd.assert().success();

            assert!(path::Path::new("./path/to/test_project/").exists())
        },
        false,
    )
}

#[test]
fn errors_on_dir_already_exists() -> () {
    test_in_tmp_dir(
        || {
            fs::create_dir("./test_project/").unwrap();

            let mut cmd: process::Command = process::Command::cargo_bin("geoff").unwrap();

            cmd.arg("create").arg("test_project");
            cmd.assert()
                .failure()
                .stderr(predicate::str::contains("test_project exists"));
        },
        false,
    )
}

#[test]
fn errors_on_parents_dont_exist() -> () {
    test_in_tmp_dir(
        || {
            let mut cmd: process::Command = process::Command::cargo_bin("geoff").unwrap();

            cmd.arg("create").arg("./path/to/test_project");
            cmd.assert()
                .failure()
                .stderr(predicate::str::contains("Parents don't exist"));
        },
        false,
    )
}

#[test]
fn errors_on_permission_denied() -> () {
    test_in_tmp_dir(
        || {
            if env::consts::OS == "windows" {
                fs::create_dir("read_only_test_dir").unwrap();

                env::set_current_dir("read_only_test_dir").expect("Can't change to read only dir");
        
                process::Command::new("Get-Item")
                    .arg("-Path")
                    .arg(".")
                    .arg("|")
                    .arg("$_.IsReadOnly = $true")
                    .output()
                    .expect("Unable to change permissions");
            } else {
                fs::create_dir("read_only_test_dir").unwrap();

                env::set_current_dir("read_only_test_dir").expect("Can't change to read only dir");
                
                process::Command::new("chmod")
                    .arg("444")
                    .arg(".")
                    .output()
                    .expect("Unable to change permissions");
            };
        
            let mut cmd: process::Command = process::Command::cargo_bin("geoff").unwrap();

            cmd.arg("create").arg("test_project");
            cmd.assert()
                .failure()
                .stderr(predicate::str::contains("Invalid permissions"));
        },
        false,
    )
}

#[test]
fn subdirectories_created() -> () {
    test_in_tmp_dir(
        || {
            let mut cmd: process::Command = process::Command::cargo_bin("geoff").unwrap();

            cmd.arg("create").arg("test_project");
            cmd.assert().success();

            let mut expected_dirs = vec![
                path::PathBuf::from("test_project/data_sources"),
                path::PathBuf::from("test_project/explorations"),
                path::PathBuf::from("test_project/models"),
                path::PathBuf::from("test_project/products"),
            ];

            let mut actual_dirs = fs::read_dir("./test_project/")
                .unwrap()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().unwrap().is_dir())
                .map(|e| e.path())
                .collect::<Vec<_>>();

            assert_eq!(expected_dirs.sort(), actual_dirs.sort())
        },
        false,
    )
}

#[test]
fn files_created() -> () {
    test_in_tmp_dir(
        || {
            let mut cmd: process::Command = process::Command::cargo_bin("geoff").unwrap();

            cmd.arg("create").arg("test_project");
            cmd.assert().success();

            let mut expected_files = vec![
                path::PathBuf::from("test_project/README.md"),
                path::PathBuf::from("test_project/project_scoping.md"),
                path::PathBuf::from("test_project/.geoff"),
            ];

            let mut actual_files = fs::read_dir("./test_project/")
                .unwrap()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().unwrap().is_file())
                .map(|e| e.path())
                .collect::<Vec<_>>();

            assert_eq!(expected_files.sort(), actual_files.sort())
        },
        false,
    )
}

#[test]
fn prints_tree() -> () {
    test_in_tmp_dir(
        || {
            let mut cmd: process::Command = process::Command::cargo_bin("geoff").unwrap();

            cmd.arg("create").arg("test_project");
            cmd.assert().success().stdout(predicate::str::contains(
                "test_project\n\
                ├─ 🖿 data_sources\n\
                ├─ 🖿 explorations\n\
                ├─ 🖿 models\n\
                ├─ 🖿 products\n\
                ├─ 🗎 README.md\n\
                └─ 🗎 project_scoping.md",
            ));
        },
        false,
    )
}
