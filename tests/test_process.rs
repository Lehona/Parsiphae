use parsiphae::{
    config::{Config, InputFile},
    processor::Parsiphae,
};

#[test_log::test]
#[ignore = "The given file does not actually typecheck, provide a more self-contained example."]
fn test_process_self_contained() {
    let config = Config {
        input_file: InputFile::SingleFile("tests/input/self_contained.d".into()),
        json: false,
    };
    let mut parsiphae = Parsiphae::new(config);
    parsiphae.process().expect("Unable to process");
}
