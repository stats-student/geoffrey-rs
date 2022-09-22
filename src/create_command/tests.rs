use std::env::set_current_dir;
use std::fs::{create_dir, read_dir};
use std::path::{Path, PathBuf};
use test_fixtures::test_in_tmp_dir;
use ptree::item::StringItem;

use super::*;

// +++++++++++++++++++++++ //
// _validate_create_result //
// +++++++++++++++++++++++ //
#[test]
#[should_panic(expected = "test_project/ exists")]
fn project_name_already_exists_error_raised() -> () {
    let create: Create = Create {
        name: PathBuf::from("./test_project/"),
        parents: false
    };

    let already_exists_err = io::Error::new(io::ErrorKind::AlreadyExists, "test_error");
    let result = Err(already_exists_err);

    create._validate_create_result(&result);
}

#[test]
#[should_panic(expected = "Parents don't exist")]
fn parents_dont_exist_error_raised() -> () {
    let create: Create = Create {
        name: PathBuf::from("./path/to/test_project/"),
        parents: false
    };

    let not_found_err = io::Error::new(io::ErrorKind::NotFound, "test_error");
    let result = Err(not_found_err);

    create._validate_create_result(&result);
}

#[test]
#[should_panic(expected = "Invalid permissions")]
fn invalid_permissions_error_raised() -> () {
    set_current_dir("/etc/").unwrap();

    let create: Create = Create {
        name: PathBuf::from("./test_project"),
        parents: false
    };

    let permission_denied_err = io::Error::new(
        io::ErrorKind::PermissionDenied,
        "test_error"
    );
    let result = Err(permission_denied_err);

    create._validate_create_result(&result);
}

// +++++++++++ //
// create_root //
// +++++++++++ //
#[test]
fn creates_new_directory_no_parents() -> (){
    test_in_tmp_dir(|| {
        let create: Create = Create {
            name: PathBuf::from("./test_project/"), 
            parents: false
        };

        create.create_root();

        assert!(Path::new("./test_project/").exists())
    }, false)
}

#[test]
fn creates_new_directory_with_parents() -> () {
    test_in_tmp_dir(|| {
        let create: Create = Create {
            name: PathBuf::from("./path/to/test_project"),
            parents: true
        };

        create.create_root();

        assert!(Path::new("./path/to/test_project").exists())
    }, false)
}

#[test]
#[should_panic(expected = "test_project/ exists")]
fn errors_on_dir_already_exists() -> () {
    test_in_tmp_dir(||{
        create_dir("./test_project/").unwrap();

        let create: Create = Create {
            name: PathBuf::from("./test_project/"), 
            parents: false
        };

        create.create_root()
    }, true)
}

#[test]
#[should_panic(expected = "Parents don't exist")]
fn errors_on_parents_dont_exist() -> () {
    test_in_tmp_dir(|| {
        let create: Create = Create {
            name: PathBuf::from("./path/to/test_project/"), 
            parents: false
        };
    
        create.create_root()
    }, true)
}

#[test]
#[should_panic(expected = "Invalid permissions")]
fn errors_on_permission_denied() -> () {
    test_in_tmp_dir(|| {
        set_current_dir("/etc/").unwrap();

        let create: Create = Create {
            name: PathBuf::from("./test_project/"), 
            parents: false
        };

        create.create_root()
    }, true)
}

// +++++++++++++++++++++ //
// create_subdirectories //
// +++++++++++++++++++++ //
#[test]
fn subdirectories_created() -> () {
    test_in_tmp_dir(||{
        create_dir("test_project").unwrap();

        let create: Create = Create {
            name: PathBuf::from("./test_project/"),
            parents: false
        };

        create.create_subdirectories();

        let mut expected_dirs = vec![
            PathBuf::from("test_project/data_sources"),
            PathBuf::from("test_project/explorations"),
            PathBuf::from("test_project/models"),
            PathBuf::from("test_project/products")
        ];

        let mut actual_dirs = read_dir("./test_project/")
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().unwrap().is_dir())
            .map(|e| e.path())
            .collect::<Vec<_>>();

        assert_eq!(expected_dirs.sort(), actual_dirs.sort())

    }, false)
}

// ++++++++++++ //
// create_files //
// ++++++++++++ //
#[test]
fn files_created() -> () {
    test_in_tmp_dir(|| {
        create_dir("test_project").unwrap();

        let create: Create = Create {
            name: PathBuf::from("./test_project/"),
            parents: false
        };

        create.create_files();

        let mut expected_files = vec![
            PathBuf::from("test_project/README.md"),
            PathBuf::from("test_project/project_scoping.md"),
            PathBuf::from("test_project/.geoff")
        ];

        let mut actual_files = read_dir("./test_project/")
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().unwrap().is_file())
            .map(|e| e.path())
            .collect::<Vec<_>>();

        assert_eq!(expected_files.sort(), actual_files.sort())

    }, false)
}

// +++++++++++ //
// create_tree //
// +++++++++++ //
#[test]
fn prints_tree() -> () {
    test_in_tmp_dir(|| {
        let gold = Style::new().color256(220);
        let hd = Style::new().color256(194);
        let bold = Style::new().bold();

        let create: Create = Create {
            name: PathBuf::from("./test_project/"),
            parents: false
        };

        let tree = create.create_tree();
        let leaves = vec![
            format!("{} data_sources", gold.apply_to("\u{1F5BF}")),
            format!("{} explorations", gold.apply_to("\u{1F5BF}")),
            format!("{} models", gold.apply_to("\u{1F5BF}")),
            format!("{} products", gold.apply_to("\u{1F5BF}")),
            format!("{} README.md", hd.apply_to("\u{1F5CE}")),
            format!("{} project_scoping.md", hd.apply_to("\u{1F5CE}"))
        ];

        assert_eq!(tree.text, format!("{}", bold.apply_to("test_project")));
        for i in 0..leaves.len() {
            assert_eq!(tree.children[i].text, leaves[i])
        }
        

    }, false)

}
