use assert_cmd::prelude::*;
use predicates::prelude::*;
use rand::Rng;
use std::env::{current_dir, set_current_dir, temp_dir};
use std::fs::{create_dir, remove_dir_all};
use std::panic;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Runs a test within a temporary directory
/// 
/// Creates a new temporary directory and changes the working directory to
/// this temporary directory.
/// Following the exectuion of the test the temporary directory and all files
/// within it are deleted.
/// 
/// # Arguments:
/// 
/// * `test` - any type that implements the FnOnce trait and returns either
///            () or a panic::UnwindSafe
fn test_in_tmp_dir<T>(test: T) -> ()
where
    T: FnOnce() -> () + panic::UnwindSafe,
{
    /// Sets up the temporary directory and changes the cwd
    /// 
    /// Creates a new temporary directory and changes the current working
    /// directory to this directory.
    /// 
    /// # Returns:
    /// 
    /// * tmp_dir - A PathBuf with the location of the temporary directory
    fn setup() -> PathBuf {
        let mut rng = rand::thread_rng();

        let mut tmp_dir: PathBuf = temp_dir();
        tmp_dir.push(format!("rust_test_{}", rng.gen::<u32>()));

        create_dir(&tmp_dir).expect("Unable to create temporary testing dir");

        set_current_dir(&tmp_dir).expect("Unable to change directory");

        tmp_dir
    }

    /// Changes to a different working directory and removes the temporary one
    /// 
    /// Changes the working directory to the one passed in the `curr-dir`
    /// argument.
    /// Following the change of directory the temporary directory and all of the
    /// contents are removed.
    /// 
    /// # Arguments:
    /// 
    /// * `change_dir` - A reference to a PathBuf containing the location of the
    ///                  directory to change to.
    /// * `temp_dir` - A reference to the temporary directory to be removed
    fn teardown(change_dir: &PathBuf, temp_dir: &PathBuf) -> () {
        set_current_dir(change_dir).expect("Unable to change directory");
        remove_dir_all(temp_dir).expect("Unable to remove the temporary testing dir");

    }

    // Setup
    let cwd: PathBuf = current_dir().expect("Unable to get current directory");
    let tmp_dir: PathBuf = setup();

    // Test
    let result = panic::catch_unwind(|| test());
    
    // Teardown
    teardown(&cwd, &tmp_dir);

    assert!(result.is_ok())
}

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
