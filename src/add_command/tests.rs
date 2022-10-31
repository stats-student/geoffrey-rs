use console::Style;
use predicates::prelude::*;
use rstest::rstest;
use std::{env, fs, path};
use test_fixtures::test_in_tmp_dir;

use super::*;

// ++++++++++++ //
// _geoff_check //
// ++++++++++++ //

#[rstest]
#[case(false, false, false)]
#[case(true, false, false)]
#[case(false, true, false)]
#[case(false, false, true)]
#[should_panic(expected = "This directory is not managed by geoff")]
fn check_not_managed_by_geoff(
    #[case] db_opt: bool,
    #[case] extract_opt: bool,
    #[case] web_opt: bool,
) -> () {
    test_in_tmp_dir(
        || {
            fs::create_dir_all("test_project/data_sources").unwrap();
            env::set_current_dir("test_project").unwrap();

            let data_source = DataSource {
                name: path::PathBuf::from("test_data_source"),
                database: db_opt,
                extract: extract_opt,
                web: web_opt,
            };

            data_source._geoff_check().unwrap();
        },
        true,
    )
}

#[rstest]
#[case(false, false, false)]
#[case(true, false, false)]
#[case(false, true, false)]
#[case(false, false, true)]
fn check_managed_by_geoff(
    #[case] db_opt: bool,
    #[case] extract_opt: bool,
    #[case] web_opt: bool,
) -> () {
    test_in_tmp_dir(
        || {
            fs::create_dir_all("test_project/data_sources").unwrap();
            env::set_current_dir("test_project").unwrap();
            fs::File::create(".geoff").unwrap();

            let data_source = DataSource {
                name: path::PathBuf::from("test_data_source"),
                database: db_opt,
                extract: extract_opt,
                web: web_opt,
            };

            data_source._geoff_check().unwrap();
        },
        false,
    )
}

// ++++++++++++++++++ //
// create_data_source //
// ++++++++++++++++++ //

#[rstest]
#[case(false, false, false)]
#[case(true, false, false)]
#[case(false, true, false)]
#[case(false, false, true)]
fn check_data_source_is_created(
    #[case] db_opt: bool,
    #[case] extract_opt: bool,
    #[case] web_opt: bool,
) -> () {
    test_in_tmp_dir(
        || {
            fs::create_dir_all("test_project/data_sources").unwrap();
            env::set_current_dir("test_project").unwrap();

            let data_source = DataSource {
                name: path::PathBuf::from("test_data_source"),
                database: db_opt,
                extract: extract_opt,
                web: web_opt,
            };

            data_source.create_data_source();

            assert!(path::Path::new("./data_sources/test_data_source").exists())
        },
        false,
    )
}

#[rstest]
#[case(false, false, false)]
#[case(true, false, false)]
#[case(false, true, false)]
#[case(false, false, true)]
#[should_panic(expected = "test_data_source exists")]
fn errors_on_dir_already_exists(
    #[case] db_opt: bool,
    #[case] extract_opt: bool,
    #[case] web_opt: bool,
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
                web: web_opt,
            };

            data_source.create_data_source();
        },
        true,
    )
}

// ++++++++++++++++++++++++++ //
// retrieve_metadata_contents //
// ++++++++++++++++++++++++++ //

#[test]
fn gets_correct_metadata_no_options() -> () {
    let data_source = DataSource {
        name: path::PathBuf::from("test_data_source"),
        database: false,
        extract: false,
        web: false,
    };

    let metadata: &str = data_source.retrieve_metadata_contents();

    let metadata_contents_fn = predicates::str::contains("# <<<data_source_name>>>");

    assert!(metadata_contents_fn.eval(metadata))
}

#[test]
fn gets_correct_metadata_database() -> () {
    let data_source = DataSource {
        name: path::PathBuf::from("test_data_source"),
        database: true,
        extract: false,
        web: false,
    };

    let metadata: &str = data_source.retrieve_metadata_contents();

    let mut expected_contents = String::from(
        "# <<<data_source_name>>>\n\
        \n\
        ## Database details\n\
        \n\
        | database name(s) | tables accessed | tables_created |\n\
        | :--------------- | :-------------- | :------------- |\n\
        | *e.g datasets* | *e.g iris* | *e.g iris_transformed* |\n\
        \n\
        ## Data dictionary\n\
        \n\
        | data field | data type | description |\n\
        | :--------- | :-------- | :---------- |\n\
        | *e.g ID* | *e.g INT* | *e.g A unique identifier for each record* |\n\
        | *e.g sepal_width* | *e.g FLOAT* | *e.g The width of the sepal part of the plant* |\n\
        \n\
        ## Key people\n\
        \n\
        | name | details |\n\
        | :--- | :------ |\n\
        | *e.g Ronald Fisher* | *Gathered the data and is the subject metter expert* |",
    );

    if env::consts::OS == "windows" {
        expected_contents = expected_contents.replace("\n", "\r\n");
    }

    let metadata_contents_fn = predicates::str::contains(expected_contents);

    assert!(metadata_contents_fn.eval(metadata))
}

#[test]
fn gets_correct_metadata_extract() -> () {
    let data_source = DataSource {
        name: path::PathBuf::from("test_data_source"),
        database: false,
        extract: true,
        web: false,
    };

    let metadata: &str = data_source.retrieve_metadata_contents();

    let mut expected_contents = String::from(
        "# <<<data_source_name>>>\n\
        \n\
        ## Extract details\n\
        \n\
        | extract file | extract query | extractor | date received |\n\
        | :----------- | :------------ | :-------- | :------------ |\n\
        | *e.g iris.csv* | *e.g SELECT * FROM iris_table* | *e.g Ronald Fisher - data collection team* | *e.g 2022-04-01* |\n\
        \n\
        ## Data dictionary\n\
        \n\
        | data field | data type | description |\n\
        | :--------- | :-------- | :---------- |\n\
        | *e.g ID* | *e.g INT* | *e.g A unique identifier for each record* |\n\
        | *e.g sepal_width* | *e.g FLOAT* | *e.g The width of the sepal part of the plant* |\n\
        \n\
        ## Key people\n\
        \n\
        | name | details |\n\
        | :--- | :------ |\n\
        | *e.g Ronald Fisher* | *Gathered the data and is the subject metter expert. Ron and the data collection team have access to the database and extracted it to a csv* |\n\
    ");

    if env::consts::OS == "windows" {
        expected_contents = expected_contents.replace("\n", "\r\n");
    }

    let metadata_contents_fn = predicates::str::contains(expected_contents);

    assert!(metadata_contents_fn.eval(metadata))
}

#[test]
fn gets_correct_metadata_web() -> () {
    let data_source = DataSource {
        name: path::PathBuf::from("test_data_source"),
        database: false,
        extract: false,
        web: true,
    };

    let metadata: &str = data_source.retrieve_metadata_contents();

    let mut expected_contents = String::from(
        "# <<<data_source_name>>>\n\
        \n\
        ## Website details\n\
        \n\
        | download file | donwload link | donwloaded on | additional details |\n\
        | :------------ | :------------ | :------------ | :----------------- |\n\
        | *e.g iris.data* | *e.g https://archive.ics.uci.edu/ml/machine-learning-databases/iris/iris.data* | *e.g 2022-01-01* | *e.g https://archive.ics.uci.edu/ml/datasets/iris* |\n\
        \n\
        ## Data dictionary\n\
        \n\
        | data field | data type | description |\n\
        | :--------- | :-------- | :---------- |\n\
        | *e.g ID* | *e.g INT* | *e.g A unique identifier for each record* |\n\
        | *e.g sepal_width* | *e.g FLOAT* | *e.g The width of the sepal part of the plant* |\n\
        \n\
        ## Key people\n\
        \n\
        | name | details |\n\
        | :--- | :------ |\n\
        | *e.g Ronald Fisher* | *Gathered the data and is the subject metter expert* |\n\
        | *e.g Michael Marshall* | *(MARSHALL%PLU@io.arc.nasa.gov) Donated the dataset to the UCI machine learning repository* |\n\
    ");

    if env::consts::OS == "windows" {
        expected_contents = expected_contents.replace("\n", "\r\n");
    }

    let metadata_contents_fn = predicates::str::contains(expected_contents);

    assert!(metadata_contents_fn.eval(metadata))
}

// +++++++++++++++++++ //
// update_placeholders //
// +++++++++++++++++++ //

#[rstest]
#[case(false, false, false)]
#[case(true, false, false)]
#[case(false, true, false)]
#[case(false, false, true)]
fn replace_placeholder_tags(
    #[case] db_opt: bool,
    #[case] extract_opt: bool,
    #[case] web_opt: bool,
) -> () {
    let data_source = DataSource {
        name: path::PathBuf::from("test_data_source"),
        database: db_opt,
        extract: extract_opt,
        web: web_opt,
    };

    let replaced_str = data_source.update_placeholders(&"# <<<data_source_name>>>");

    assert_eq!(replaced_str, "# test_data_source")
}

// +++++++++++++++ //
// create_metadata //
// +++++++++++++++ //

#[rstest]
#[case(false, false, false)]
#[case(true, false, false)]
#[case(false, true, false)]
#[case(false, false, true)]
fn metadata_file_created(
    #[case] db_opt: bool,
    #[case] extract_opt: bool,
    #[case] web_opt: bool,
) -> () {
    test_in_tmp_dir(
        || {
            fs::create_dir_all("test_project/data_sources").unwrap();
            env::set_current_dir("test_project").unwrap();
            fs::create_dir("data_sources/test_data_source").unwrap();

            let data_source = DataSource {
                name: path::PathBuf::from("test_data_source"),
                database: db_opt,
                extract: extract_opt,
                web: web_opt,
            };

            data_source.create_metadata(&String::from("# test_data_source"));

            assert!(path::Path::new("data_sources/test_data_source/metadata.md").exists())
        },
        false,
    )
}

// +++++++++++ //
// create_tree //
// +++++++++++ //
#[rstest]
#[case(false, false, false)]
#[case(true, false, false)]
#[case(false, true, false)]
#[case(false, false, true)]
fn creates_tree(#[case] db_opt: bool, #[case] extract_opt: bool, #[case] web_opt: bool) -> () {
    test_in_tmp_dir(
        || {
            let gold = Style::new().color256(220);
            let hd = Style::new().color256(194);

            let data_source = DataSource {
                name: path::PathBuf::from("test_data_source"),
                database: db_opt,
                extract: extract_opt,
                web: web_opt,
            };

            let tree = data_source.create_tree();

            assert_eq!(
                tree.text,
                format!("{} data_sources", gold.apply_to("\u{1F5BF}"))
            );
            assert_eq!(
                tree.children[0].text,
                format!("{} test_data_source", gold.apply_to("\u{1F5BF}"))
            );
            assert_eq!(
                tree.children[0].children[0].text,
                format!("{} metadata.md", hd.apply_to("\u{1F5CE}"))
            );
        },
        false,
    )
}
