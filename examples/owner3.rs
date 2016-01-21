struct Foo {
    x: i32,
}

fn inc(f: &mut Foo) {
    f.x += 1;
}

fn main() {
    let mut a = Foo { x: 123 };
    let b = &mut a;
    inc(&mut a);
    println!("{}", b.x);
}
