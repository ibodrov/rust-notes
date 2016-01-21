enum Foo {
    Apple,
    Orange{ size: u32 },
}

fn main() {
    let x = Foo::Orange { size: 100 };
    match x {
        Foo::Apple => println!("Boo! Android is better!"),
        Foo::Orange{ size } => println!("An orange of size {}", size),
    }
}
// компилятор понимает, что Apple не используется
