fn get_something() -> Option<i32> {
    Some(123)
}

fn main() {
    let x = match get_something() {
        Some(x) => x + 100,
        None => panic!("OMG!"),
    };

    println!("got {}", x);
}
