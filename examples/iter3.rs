fn main() {
    let n: Vec<_> = (0..10)
        .cycle()
        .take(1000)
        .collect();

    println!("{}", n.len());
}
