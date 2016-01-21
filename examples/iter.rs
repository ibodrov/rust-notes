fn main() {
    let n = (0..10)
        .map(|x| x + 100)
        // map делает move в данном случае

        .filter(|&x| x % 2 == 0)
        // filter принимает ref

        .fold(0, |sum, x| sum + x)
        // fold делает move
        ;

    println!("{}", n);
}
