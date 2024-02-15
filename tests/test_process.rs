use parsiphae::{
    config::{Config, InputFile},
    processor::Parsiphae,
};

#[test]
fn test_process_self_contained() {
    let config = Config {
        input_file: InputFile::SingleFile("tests/input/self_contained.d".into()),
        json: false,
    };
    Parsiphae::process(config).expect("Unable to process");
}
