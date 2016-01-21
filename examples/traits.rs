trait DoesSomething {
    fn do_something(&self);
}

enum Fruit {
    Apple,
    Orange,
}

impl DoesSomething for Fruit {
    fn do_something(&self) {
        match *self {
            Fruit::Apple => println!("11111"),
            Fruit::Orange => println!("22222"),
        }
    }
}

fn main() {
    let x = Fruit::Apple;
    x.do_something();
}
