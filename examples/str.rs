fn main() {
    let a = "Hello, world";
    // &'static str
    // str - статически аллоцированная строка неизвестной длины
    // иммутабельна (без unsafe)
    // имеет смысл в только виде &str - slice (pointer + length)

    let mut b = String::from(a);
    // строка выделенная на куче, может меняться

    b = b + "!";

    println!("{}", b);
}
