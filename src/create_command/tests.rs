use predicates::prelude::*;
use std::{env, fs, path};
use test_fixtures::test_in_tmp_dir;

use super::*;

// +++++++++++++++++++++++ //
// _validate_create_result //
// +++++++++++++++++++++++ //
#[test]
#[should_panic(expected = "test_project/ exists")]
fn project_name_already_exists_error_raised() -> () {
    let create: Create = Create {
        name: path::PathBuf::from("./test_project/"),
        parents: false,
    };

    let already_exists_err = io::Error::new(io::ErrorKind::AlreadyExists, "test_error");
    let result = Err(already_exists_err);

    create.validate_create_folder_result(&create.name, &result);
}

#[test]
#[should_panic(expected = "Parents don't exist")]
fn parents_dont_exist_error_raised() -> () {
    let create: Create = Create {
        name: path::PathBuf::from("./path/to/test_project/"),
        parents: false,
    };

    let not_found_err = io::Error::new(io::ErrorKind::NotFound, "test_error");
    let result = Err(not_found_err);

    create.validate_create_folder_result(&create.name, &result);
}

#[test]
#[should_panic(expected = "Invalid permissions")]
fn invalid_permissions_error_raised() -> () {
    let create: Create = Create {
        name: path::PathBuf::from("./test_project"),
        parents: false,
    };

    let permission_denied_err = io::Error::new(io::ErrorKind::PermissionDenied, "test_error");
    let result = Err(permission_denied_err);

    create.validate_create_folder_result(&create.name, &result);
}

#[test]
#[should_panic(expected = "Unknown error")]
fn generic_error_raised() -> () {
    let create: Create = Create {
        name: path::PathBuf::from("./test_project"),
        parents: false,
    };

    let invalid_data_err = io::Error::new(io::ErrorKind::InvalidData, "test_error");
    let result = Err(invalid_data_err);

    create.validate_create_folder_result(&create.name, &result);
}

// +++++++++++ //
// create_root //
// +++++++++++ //
#[test]
fn creates_new_directory_no_parents() -> () {
    test_in_tmp_dir(
        || {
            let create: Create = Create {
                name: path::PathBuf::from("./test_project/"),
                parents: false,
            };

            create.create_root();

            assert!(path::Path::new("./test_project/").exists())
        },
        false,
    )
}

#[test]
fn creates_new_directory_with_parents() -> () {
    test_in_tmp_dir(
        || {
            let create: Create = Create {
                name: path::PathBuf::from("./path/to/test_project"),
                parents: true,
            };

            create.create_root();

            assert!(path::Path::new("./path/to/test_project").exists())
        },
        false,
    )
}

#[test]
#[should_panic(expected = "test_project/ exists")]
fn errors_on_dir_already_exists() -> () {
    test_in_tmp_dir(
        || {
            fs::create_dir("./test_project/").unwrap();

            let create: Create = Create {
                name: path::PathBuf::from("./test_project/"),
                parents: false,
            };

            create.create_root()
        },
        true,
    )
}

#[test]
#[should_panic(expected = "Parents don't exist")]
fn errors_on_parents_dont_exist() -> () {
    test_in_tmp_dir(
        || {
            let create: Create = Create {
                name: path::PathBuf::from("./path/to/test_project/"),
                parents: false,
            };

            create.create_root()
        },
        true,
    )
}

#[test]
#[should_panic(expected = "Invalid permissions")]
fn errors_on_permission_denied() -> () {
    test_in_tmp_dir(
        || {
            if env::consts::OS == "windows" {
                // FIXME: Unable to create a directory in windows that causes a
                //        permissions error
                panic!("Invalid permissions");
            } else {
                // FIXME: Shouldn't be reliant on a system created folder
                env::set_current_dir("/etc/").expect("Can't change to read only dir");
            };

            let create: Create = Create {
                name: path::PathBuf::from("./test_project/"),
                parents: false,
            };

            create.create_root()
        },
        true,
    )
}

// +++++++++++++++++++++ //
// create_subdirectories //
// +++++++++++++++++++++ //
#[test]
fn subdirectories_created() -> () {
    test_in_tmp_dir(
        || {
            fs::create_dir("test_project").unwrap();

            let create: Create = Create {
                name: path::PathBuf::from("./test_project/"),
                parents: false,
            };

            create.create_subdirectories();

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

// ++++++++++++ //
// create_files //
// ++++++++++++ //
#[test]
fn files_created() -> () {
    test_in_tmp_dir(
        || {
            fs::create_dir("test_project").unwrap();

            let create: Create = Create {
                name: path::PathBuf::from("./test_project/"),
                parents: false,
            };

            create.create_files();

            let mut expected_files = vec![
                path::PathBuf::from("test_project/README.md"),
                path::PathBuf::from("test_project/project_scoping.md"),
                path::PathBuf::from("test_project/geoff.toml"),
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

// ++++++++++++++++++++ //
// replace_placeholders //
// ++++++++++++++++++++ //
#[test]
fn readme_proj_scope_placeholders_replaced() -> () {
    test_in_tmp_dir(
        || {
            fs::create_dir("test_project").unwrap();

            let create: Create = Create {
                name: path::PathBuf::from("./test_project/"),
                parents: false,
            };

            let files = collections::HashMap::from([
                ("README.md", include_str!("../templates/root/README.md")),
                (
                    "project_scoping.md",
                    include_str!("../templates/root/project_scoping.md"),
                ),
            ]);

            for (_, contents) in files.iter() {
                let updated_contents = create._update_placeholders(contents);

                let file_contains = predicates::str::contains("# test_project");

                assert!(file_contains.eval(&updated_contents))
            }
        },
        false,
    )
}

// +++++++++++ //
// create_tree //
// +++++++++++ //
#[test]
fn creates_tree() -> () {
    test_in_tmp_dir(
        || {
            let gold = Style::new().color256(220);
            let hd = Style::new().color256(194);
            let bold = Style::new().bold();

            let create: Create = Create {
                name: path::PathBuf::from("./test_project/"),
                parents: false,
            };

            let tree = create.create_tree();
            let leaves = vec![
                format!("{} data_sources", gold.apply_to("\u{1F5BF}")),
                format!("{} explorations", gold.apply_to("\u{1F5BF}")),
                format!("{} models", gold.apply_to("\u{1F5BF}")),
                format!("{} products", gold.apply_to("\u{1F5BF}")),
                format!("{} README.md", hd.apply_to("\u{1F5CE}")),
                format!("{} project_scoping.md", hd.apply_to("\u{1F5CE}")),
            ];

            assert_eq!(tree.text, format!("{}", bold.apply_to("./test_project/")));
            for i in 0..leaves.len() {
                assert_eq!(tree.children[i].text, leaves[i])
            }
        },
        false,
    )
}
