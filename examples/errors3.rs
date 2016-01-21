enum MyError {
    EveryoneDied,
    AllIsLost,
}

fn perform_something() -> Result<i32, MyError> {
    Ok(123)
}

fn main() {
    if let Ok(x) = perform_something() {
        println!("got {}", x);
    } else {
        println!("omg!");
    }
}
