use module_01::cwc::ufc_graph;

#[test]
fn test_lang_weights() {
    let expected_output:i32 = 0;
    let output:i32 = ufc_graph();
    assert_eq!(output, expected_output);
}
