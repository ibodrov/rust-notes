struct Foo {
    x: i32,
}

fn fun1(f: Foo) {
    println!("fun1: {}", f.x);
}

fn fun2(f: Foo) {
    println!("fun2: {}", f.x);
}

fn main() {
    let a = Foo { x: 123 };
    fun1(a);
    fun2(a);
    // 16:10: 16:11 error: use of moved value: `a` [E0382]
    // 16     fun2(a);
    //             ^
    // 16:10: 16:11 help: run `rustc --explain E0382` to see a detailed explanation
    // 15:10: 15:11 note: `a` moved here because it has type `Foo`, which is non-copyable
    // 15     fun1(a);
    //             ^
}
