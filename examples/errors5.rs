#[derive(Debug)]
enum MyError {
}

fn perform_something() -> Result<i32, MyError> {
    Ok(123)
}

fn main() {
    let x = perform_something().unwrap();
    // Ok(x) или panic!

    let y = perform_something().expect("OMG!");
    // Ok(x) или panic! с сообщением

    let z = perform_something()
        .or_else(|_| -> Result<i32, MyError> { Ok(777) })
        .unwrap();
    // Ok(x) или Ok(777) или panic!

    println!("{}, {}, {}", x, y, z);
}
