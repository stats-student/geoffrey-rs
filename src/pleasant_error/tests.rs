use rstest::rstest;
use std::{io, path};

use super::*;

#[test]
#[should_panic(expected = "test_data exists")]
fn pleasant_error_name_already_exists_error_raised() -> () {
    struct TestData {
        name: path::PathBuf,
    }

    impl PleasantErrorHandler for TestData {}

    let test_data = TestData {
        name: path::PathBuf::from("test_data"),
    };

    let already_exists_err = io::Error::new(io::ErrorKind::AlreadyExists, "test_error");

    test_data.already_exists_err_msg(&test_data.name, &already_exists_err);
}

#[test]
#[should_panic(expected = "Parents don't exist")]
fn pleasant_error_parents_dont_exist_error_raised() -> () {
    struct TestData {
        name: path::PathBuf,
    }

    impl PleasantErrorHandler for TestData {}

    let test_data = TestData {
        name: path::PathBuf::from("test_data"),
    };

    let not_found_err = io::Error::new(io::ErrorKind::NotFound, "test_error");

    test_data.not_found_err_msg(&test_data.name, &not_found_err);
}

#[test]
#[should_panic(expected = "Invalid permissions")]
fn pleasant_error_invalid_permissions_error_raised() -> () {
    struct TestData {
        name: path::PathBuf,
    }

    impl PleasantErrorHandler for TestData {}

    let test_data = TestData {
        name: path::PathBuf::from("test_data"),
    };

    let permission_denied_err = io::Error::new(io::ErrorKind::PermissionDenied, "test_error");

    test_data.permission_denied_err_msg(&test_data.name, &permission_denied_err);
}

#[test]
#[should_panic(expected = "Unknown error")]
fn pleasant_error_generic_error_raised() -> () {
    struct TestData {
        name: path::PathBuf,
    }

    impl PleasantErrorHandler for TestData {}

    let test_data = TestData {
        name: path::PathBuf::from("test_data"),
    };

    let invalid_data_err = io::Error::new(io::ErrorKind::InvalidData, "test_error");

    test_data.generic_err_msg(&invalid_data_err);
}

#[rstest(
    kind,
    #[should_panic(expected = "test_data exists")]
    case::panic_with_message(io::ErrorKind::AlreadyExists),
    #[should_panic(expected = "Parents don't exist")]
    case::panic_with_message(io::ErrorKind::NotFound),
    #[should_panic(expected = "Invalid permissions")]
    case::panic_with_message(io::ErrorKind::PermissionDenied),
    #[should_panic(expected = "Unknown error")]
    case::panic_with_message(io::ErrorKind::InvalidData),
)]
fn pleasant_error_validate_error_kinds_raised(kind: io::ErrorKind) -> () {
    struct TestData {
        name: path::PathBuf,
    }

    impl PleasantErrorHandler for TestData {}

    let test_data = TestData {
        name: path::PathBuf::from("test_data"),
    };

    let err = io::Error::new(kind, "test_error");
    let result = Err(err);

    test_data.validate_create_folder_result(&test_data.name, &result);
}
