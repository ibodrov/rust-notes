struct Foo {
    x: i32,
}

fn fun1(f: &Foo) {
    println!("fun1: {}", f.x);
}

fn fun2(f: &Foo) {
    println!("fun2: {}", f.x);
}

fn main() {
    let a = Foo { x: 123 };
    fun1(&a);
    fun2(&a);
}
