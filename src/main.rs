fn main() {
    println!("Hello, world!");
}

#[test]
fn test_simple() {
    println!("Hello Test")
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn test_add() {
    let result = add(1, 2);
    assert_eq!(result, 3, "1 + 2 should be 3")
}

fn start_application(host: &str, port: u16) {
    if host == "localhost" {
        panic!("You can't use localhost as host!")
    } else {
        println!("Starting application on {}:{}", host, port)
    }
}

#[test]
#[should_panic]
fn test_start_application() {
    start_application("localhost", 8080)
}