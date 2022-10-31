use assert_cmd::prelude::*;
use predicates::prelude::*;
use rstest::rstest;
use std::{env, fs, path, process};
use test_fixtures::test_in_tmp_dir;

#[rstest]
#[case("")]
#[case("--database")]
#[case("-d")]
#[case("--extract")]
#[case("-e")]
#[case("--web")]
#[case("-w")]
fn creates_new_default_data_source(#[case] option: &str) -> () {
    test_in_tmp_dir(
        || {
            fs::create_dir_all("./test_project/data_sources").unwrap();
            env::set_current_dir("test_project").unwrap();

            let mut cmd = process::Command::cargo_bin("geoff").unwrap();

            cmd.arg("add").arg("data-source").arg("test_data_source");

            if !option.is_empty() {
                cmd.arg(option);
            }

            cmd.assert().success();

            assert!(path::Path::new("./data_sources/test_data_source").exists())
        },
        false,
    )
}

#[rstest]
#[case("")]
#[case("--database")]
#[case("-d")]
#[case("--extract")]
#[case("-e")]
#[case("--web")]
#[case("-w")]
fn creates_files_default_data_source(#[case] option: &str) -> () {
    test_in_tmp_dir(
        || {
            fs::create_dir_all("./test_project/data_sources").unwrap();
            env::set_current_dir("test_project").unwrap();

            let mut cmd = process::Command::cargo_bin("geoff").unwrap();

            cmd.arg("add").arg("data-source").arg("test_data_source");
            cmd.assert().success();

            if !option.is_empty() {
                cmd.arg(option);
            }

            assert!(path::Path::new("./data_sources/test_data_source/metadata.md").exists())
        },
        false,
    )
}

#[rstest]
#[case("")]
#[case("--database")]
#[case("-d")]
#[case("--extract")]
#[case("-e")]
#[case("--web")]
#[case("-w")]
fn prints_default_data_source_tree(#[case] option: &str) -> () {
    test_in_tmp_dir(
        || {
            fs::create_dir_all("./test_project/data_sources").unwrap();
            env::set_current_dir("test_project").unwrap();

            let mut cmd = process::Command::cargo_bin("geoff").unwrap();

            cmd.arg("add").arg("data-source").arg("test_data_source");

            if !option.is_empty() {
                cmd.arg(option);
            }

            cmd.assert().success().stdout(predicates::str::contains(
                "🖿 data_sources\n└─ 🖿 test_data_source\n   └─ 🗎 metadata.md",
            ));
        },
        false,
    )
}

// ++++++++++ //
// No options //
// ++++++++++ //
#[test]
fn default_metadata_has_only_title() -> () {
    test_in_tmp_dir(
        || {
            fs::create_dir_all("./test_project/data_sources").unwrap();
            env::set_current_dir("test_project").unwrap();

            let mut cmd = process::Command::cargo_bin("geoff").unwrap();

            cmd.arg("add").arg("data-source").arg("test_data_source");
            cmd.assert().success();

            let contents =
                fs::read_to_string("./data_sources/test_data_source/metadata.md").unwrap();

            let file_contains = if env::consts::OS == "windows" {
                predicates::str::is_match("^# test_data_source\r\n$").unwrap()
            } else {
                predicates::str::is_match("^# test_data_source\n$").unwrap()
            };

            assert!(file_contains.eval(&contents))
        },
        false,
    )
}

// ++++++++ //
// Database //
// ++++++++ //
#[rstest]
#[case("--database")]
#[case("-d")]
fn database_metadata_contents_correct(#[case] option: &str) -> () {
    test_in_tmp_dir(
        || {
            fs::create_dir_all("./test_project/data_sources").unwrap();
            env::set_current_dir("test_project").unwrap();

            let mut cmd = process::Command::cargo_bin("geoff").unwrap();

            cmd.arg("add")
                .arg("data-source")
                .arg(option)
                .arg("test_data_source");
            cmd.assert().success();

            let contents =
                fs::read_to_string("./data_sources/test_data_source/metadata.md").unwrap();
            assert!(predicates::str::contains("# test_data_source").eval(&contents));
            assert!(predicates::str::contains("## Database details").eval(&contents));
            assert!(predicates::str::contains("## Data dictionary").eval(&contents));
            assert!(predicates::str::contains("## Key people").eval(&contents))
        },
        false,
    )
}

// +++++++ //
// Extract //
// +++++++ //
#[rstest]
#[case("--extract")]
#[case("-e")]
fn extract_metadata_contents_correct(#[case] option: &str) -> () {
    test_in_tmp_dir(
        || {
            fs::create_dir_all("./test_project/data_sources").unwrap();
            env::set_current_dir("test_project").unwrap();

            let mut cmd = process::Command::cargo_bin("geoff").unwrap();

            cmd.arg("add")
                .arg("data-source")
                .arg(option)
                .arg("test_data_source");
            cmd.assert().success();

            let contents =
                fs::read_to_string("./data_sources/test_data_source/metadata.md").unwrap();
            assert!(predicates::str::contains("# test_data_source").eval(&contents));
            assert!(predicates::str::contains("## Extract details").eval(&contents));
            assert!(predicates::str::contains("## Data dictionary").eval(&contents));
            assert!(predicates::str::contains("## Key people").eval(&contents))
        },
        false,
    )
}

// +++ //
// Web //
// +++ //
#[rstest]
#[case("--web")]
#[case("-w")]
fn web_metadata_contents_correct(#[case] option: &str) -> () {
    test_in_tmp_dir(
        || {
            fs::create_dir_all("./test_project/data_sources").unwrap();
            env::set_current_dir("test_project").unwrap();

            let mut cmd = process::Command::cargo_bin("geoff").unwrap();

            cmd.arg("add")
                .arg("data-source")
                .arg(option)
                .arg("test_data_source");
            cmd.assert().success();

            let contents =
                fs::read_to_string("./data_sources/test_data_source/metadata.md").unwrap();
            assert!(predicates::str::contains("# test_data_source").eval(&contents));
            assert!(predicates::str::contains("## Website details").eval(&contents));
            assert!(predicates::str::contains("## Data dictionary").eval(&contents));
            assert!(predicates::str::contains("## Key people").eval(&contents))
        },
        false,
    )
}

// ++++++++++++++++ //
// Multiple options //
// ++++++++++++++++ //
#[rstest]
#[case("--web --database")]
#[case("--web --extract")]
#[case("--extract --database")]
#[case("--web --extract --database")]
fn multiple_options_should_error(#[case] option: &str) -> () {
    test_in_tmp_dir(
        || {
            fs::create_dir_all("./test_project/data_sources").unwrap();
            env::set_current_dir("test_project").unwrap();

            let mut cmd = process::Command::cargo_bin("geoff").unwrap();

            cmd.arg("add").arg("data-source").arg("test_data_source");

            let options: std::vec::Vec<&str> = option.split(" ").collect();
            for opt in options.iter() {
                cmd.arg(opt);
            }

            cmd.assert().failure();
        },
        false,
    )
}
