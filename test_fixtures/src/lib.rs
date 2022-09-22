use rand::Rng;
use std::env::{current_dir, set_current_dir, temp_dir};
use std::fs::{create_dir, remove_dir_all};
use std::panic;
use std::path::PathBuf;


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
pub fn test_in_tmp_dir<T>(test: T, should_panic: bool) -> ()
where
    T: FnOnce() -> () + panic::UnwindSafe,
{
    // Setup
    let cwd: PathBuf = current_dir().expect("Unable to get current directory");
    let tmp_dir: PathBuf = create_random_tmp_dir().unwrap();

    set_current_dir(&tmp_dir).expect("Unable to change directory");

    // Test
    let result = panic::catch_unwind(|| test());
    
    // Teardown
    teardown(&cwd, &tmp_dir);

    if should_panic {
        match result {
            Ok(_) => assert!(result.is_ok()),
            Err(err) => panic::resume_unwind(err)
        }
    } else {
        assert!(result.is_ok())
    }
}

/// Sets up the temporary directory
/// 
/// Creates a new temporary directory with a random name to avoid conflict
/// 
/// # Returns:
/// 
/// * tmp_dir - A PathBuf with the location of the temporary directory
fn create_random_tmp_dir() -> std::io::Result<PathBuf> {
    let mut rng = rand::thread_rng();

    let mut tmp_dir: PathBuf = temp_dir();
    tmp_dir.push(format!("rust_test_{}", rng.gen::<u32>()));

    create_dir(&tmp_dir).expect("Unable to create temporary testing dir");

    Ok(tmp_dir)
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