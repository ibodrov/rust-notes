struct Foo {
    x: i32,
}

struct Bar(i32, i32, i32);

fn main() {
    let a = Foo { x: 123 };
    println!("{}", a.x);
    let b = Bar(0, 1, 2);
    println!("{}, {}, {}", b.0, b.1, b.2);
}
