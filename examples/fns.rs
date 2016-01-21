fn foo() {
}

fn bar(x: i32) -> i32 {
    x + 1
}

fn one<F: Fn(i32) -> i32>(x: i32, f: F) -> i32 {
    f(x) + 1
}

fn two<F>(x: i32, f: F) -> i32
    where F : Fn(i32) -> i32 {
    f(x) + 1
}

fn make_adder(i: i32) -> Box<Fn(i32) -> i32> {
    // пока не доделают abstract return types,
    // только через хип
    Box::new(move |x| x + i)
}

fn main() {
    foo();
    bar(10);
    one(10, |x| x + 2);
    two(10, |x| x + 2);

    let add_two = make_adder(2);
    add_two(3); 
}
