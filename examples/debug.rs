#[inline(never)]
#[no_mangle]
fn do_something(i: i32) -> i32 {
    i + 1
}

fn main() {
    let mut sum = 0;
    for _ in 0..100000 {
        sum += do_something(10);
    }
    println!("{}", sum);
}
