use module_01::freq_count::freq_count;

#[test]
fn test_collections() {
    let expected_output:i32 = 0;
    let output:i32 = freq_count();
    assert_eq!(output, expected_output);
}
