fn main() {
    let n = (0..10)
        .fold(0, |sum, x| sum + x);

    std::process::exit(n);
}
