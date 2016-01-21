#![allow(dead_code)]

struct Child;

struct Parent<'a> {
    child: &'a Child,
}

fn main() {
    let x = 1;
    {
        let y = 2;
    }

    let c = Child;
    let _p = Parent { child: &c };
}
