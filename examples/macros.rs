macro_rules! foo {
    (weather is $e:expr) => (println!("Weather is known to be {}", stringify!($e)))
}

fn main() {
    foo!(weather is fine);
}
