fn main() {
    let n = (0..10)
        .cycle()
        .take(1000)
        .fold(0, |sum, x| sum + x);

    println!("{}", n);
}
