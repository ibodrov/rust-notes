fn main() {
    let x = 0;
    x += 1;
    // 3:5: 3:11 error: re-assignment of immutable variable `x` [E0384]
    // 3     x += 1;
    //       ^~~~~~
    //
    // 2:10 note: prior assignment occurs here
    // 2     let x = 0;
    //           ^

    println!("{}", x);
}
