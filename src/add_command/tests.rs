use rstest::{rstest};
use std::{env, fs, path};
use test_fixtures::test_in_tmp_dir;

use super::*;


#[rstest]
#[case(false, false, false)]
#[case(true, false, false)]
#[case(false, true, false)]
#[case(false, false, true)]
#[should_panic(expected = "This directory is not managed by geoff")]
fn check_if_managed_by_geoff (
    #[case] db_opt: bool, #[case] extract_opt: bool, #[case] web_opt: bool
) -> () {
    test_in_tmp_dir(|| {
            fs::create_dir_all("test_project/data_sources").unwrap();
            env::set_current_dir("test_project").unwrap();

            let data_source = DataSource {
                name: path::PathBuf::from("test_data_source"),
                database: db_opt,
                extract: extract_opt,
                web: web_opt
            };

            data_source._geoff_check().unwrap();
        },
        true
    )
}


#[rstest]
#[case(false, false, false)]
#[case(true, false, false)]
#[case(false, true, false)]
#[case(false, false, true)]
fn check_data_source_is_created(
    #[case] db_opt: bool, #[case] extract_opt: bool, #[case] web_opt: bool 
) -> () {
    test_in_tmp_dir(||{
        fs::create_dir_all("test_project/data_sources").unwrap();
        env::set_current_dir("test_project").unwrap();

        let data_source = DataSource {
            name: path::PathBuf::from("test_data_source"),
            database: db_opt,
            extract: extract_opt,
            web: web_opt
        };

        data_source.create_data_source();

        assert!(path::Path::new("./data_sources/test_data_source").exists())
    },
    false)
}

#[rstest]
#[case(false, false, false)]
#[case(true, false, false)]
#[case(false, true, false)]
#[case(false, false, true)]
#[should_panic(expected = "test_data_source exists")]
fn errors_on_dir_already_exists(
    #[case] db_opt: bool, #[case] extract_opt: bool, #[case] web_opt: bool 
) -> () {
    test_in_tmp_dir(
        || {
            fs::create_dir_all("test_project/data_sources").unwrap();
            env::set_current_dir("test_project").unwrap();
            fs::File::create("data_sources/test_data_source").unwrap();

            let data_source = DataSource {
                name: path::PathBuf::from("test_data_source"),
                database: db_opt,
                extract: extract_opt,
                web: web_opt
            };

            data_source.create_data_source();

        },
        true,
    )
}


