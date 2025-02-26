use module_02::fruitsalad::{default_fruitlist,read_fruitlist,create_fruitsalad,display_fruitsalad,demo_fruitsalad};


#[test]
fn test_default_fruitlist() {
    let expected_output:usize = 10;
    let fruit_list = default_fruitlist();
    let output:usize = fruit_list.len();
    assert_eq!(output, expected_output);
}
#[test]
fn test_read_fruitlist() {
    let expected_output:usize = 11;
    let fruit_list = read_fruitlist("fruitlist.csv");
    let output:usize = fruit_list.len();
    assert_eq!(output, expected_output);
}
#[test]
fn test_create_fruitsalad() {
    let expected_output:usize = 10;
    let mut fruit_list = default_fruitlist();
    let fruit_salad = create_fruitsalad(&fruit_list);
    let output:usize = fruit_salad.len();
    assert_eq!(output, expected_output);
}
#[test]
fn test_display_fruitsalad() {
    let expected_output:i32 = 0;
    let mut fruit_list = default_fruitlist();
    let fruit_salad = create_fruitsalad(&fruit_list);
    display_fruitsalad(fruit_salad);
    let output:usize = fruit_salad.len();
    assert_eq!(output, expected_output);
}
#[test]
fn test_demo_fruitsalad() {
    let expected_output:i32 = 0;
    let output:i32 = demo_fruitsalad();
    assert_eq!(output, expected_output);
}
