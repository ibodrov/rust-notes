enum MyError {
    EveryoneDied,
    AllIsLost,
}

fn perform_something() -> Result<i32, MyError> {
    Ok(123)
}

fn foo() -> Result<i32, MyError> {
    let x = try!(perform_something());
    let y = try!(perform_something());
    Ok(x + y)
}

fn main() {
    match foo() {
        Ok(x) => println!("{}", x),
        Err(_) => println!("OMG"),
    }
}
