use module_01::lang_weights::lang_weights;

#[test]
fn test_lang_weights() {
    let expected_output:i32 = 0;
    let output:i32 = lang_weights();
    assert_eq!(output, expected_output);
}
