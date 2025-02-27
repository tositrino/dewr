use module_03::dp::{demo_dp};

#[test]
fn test_dp() {
    let expected_output:i32 = 0;
    let output:i32 = demo_dp();
    assert_eq!(output, expected_output);
}
