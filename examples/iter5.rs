#[derive(Debug)]
enum MyError {}

fn produce_something() -> Result<i32, MyError> {
    Ok(1)
}

fn main() {
    let n: Result<Vec<i32>, MyError> = (0..10)
        .map(|_| produce_something())
        .collect();
    // 10 элеметов
    // для каждого вызывается produce_something()
    // результат собирается в коллекцию
    // работает потому, что есть impl FromIterator for Result

    let v = n.unwrap();
    // Vec<i32> или panic!

    println!("{}", v.len());
}
