#[test]
fn test_say_hello() {
    let result = hello::say_hello("Eko");
    assert_eq!(result, "Hello Eko!")
}