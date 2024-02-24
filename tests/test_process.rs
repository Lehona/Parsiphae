use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;

use parsiphae::{
    config::{Config, InputFile},
    processor::Parsiphae,
};

#[test_log::test]
fn test_process_self_contained() {
    let config = Config {
        input_file: InputFile::SingleFile("tests/input/self_contained.d".into()),
        json: false,
    };
    let mut parsiphae = Parsiphae::new(config);
    parsiphae.process().expect("Unable to process");
}

#[test_log::test]
fn test_constants() {
    let config = Config {
        input_file: InputFile::SingleFile("tests\\input\\collection\\constants.d".into()),
        json: false,
    };
    let mut parsiphae = Parsiphae::new(config);
    parsiphae.process().expect("Unable to process");
}

#[test_log::test]
fn test_readyspell() {
    let config = Config {
        input_file: InputFile::SingleFile(
            "tests\\input\\bereinigte_skripte\\AI\\Magic\\B_ReadySpell.d".into(),
        ),
        json: false,
    };
    let mut parsiphae = Parsiphae::new(config);
    parsiphae
        .process()
        .inspect_err(|e| e.render(&parsiphae.config, &parsiphae.file_db))
        .expect("Unable to process");
}

fn test_list_of_files(files: &[String]) {
    let mut src = tempfile::NamedTempFile::new_in("tests\\input\\collection").unwrap();
    write!(src, "{}\r\n", files.join("\r\n")).unwrap();
    src.seek(SeekFrom::Start(0)).unwrap();
    let config = Config {
        input_file: InputFile::Src(src.path().to_owned()),
        json: false,
    };
    let mut parsiphae = Parsiphae::new(config);
    match parsiphae.process() {
        Err(e) => {
            e.render(&parsiphae.config, &parsiphae.file_db);
            src.close().unwrap();
        }
        Ok(_) => src.close().unwrap(),
    }
}

#[test_log::test]
fn test_inremental() {
    let files = ["Constants.d", "Classes.d"];
    let mut incremental_files = vec![];
    for file in files {
        incremental_files.push(format!("{file}"));
        test_list_of_files(&incremental_files);
    }
}

#[test_log::test]
fn test_bereinigte_skripte() {
    let config = Config {
        input_file: InputFile::Src("tests\\input\\bereinigte_skripte\\Gothic.src".into()),
        json: false,
    };
    let mut parsiphae = Parsiphae::new(config);
    parsiphae
        .process()
        .inspect_err(|e| e.render(&parsiphae.config, &parsiphae.file_db))
        .expect("Unable to process");
}

#[test_log::test]
fn test_error() {
    let config = Config {
        input_file: InputFile::SingleFile("tests/input/errors.d".into()),
        json: false,
    };
    let mut parsiphae = Parsiphae::new(config);
    parsiphae
        .process()
        .inspect_err(|e| e.render(&parsiphae.config, &parsiphae.file_db))
        .expect("Unable to process");
}
