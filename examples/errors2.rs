enum MyError {
    EveryoneDied,
    AllIsLost,
}

fn perform_something() -> Result<i32, MyError> {
    Ok(123)
}

fn main() {
    let x = match perform_something() {
        Ok(x) => x * 123,
        Err(_) => panic!("OMG!"),
    };

    println!("got {}", x);
}
