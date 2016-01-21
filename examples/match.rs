fn main() {
    let x = 100;
    match x {
        0 => println!("nope"),
        90...100 => println!("yep"),
        _ => println!("nope"),
    }
}
