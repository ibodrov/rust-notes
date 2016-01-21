fn dot_product(x: &[i32], y: &[i32]) -> i32 {
    x.iter().zip(y.iter())
        .fold(0, |sum, (&ex, &ey)| sum + (ex * ey))
}


fn main() {
    let xs = vec![0; 1024];
    let ys = vec![0; 1024];

    let s = dot_product(&xs, &ys);
    std::process::exit(s);
}
