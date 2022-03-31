#[test]
fn test_process_self_contained() {
    parsiphae::processor::process_single_file("tests/input/self_contained.d")
        .expect("Unable to process");
}
