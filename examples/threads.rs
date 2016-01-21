use std::thread;

fn main() {
    let mut data = vec![0, 1, 2];

    for i in 0..3 {
        thread::spawn(move || {
            data[i] += 1;
        });
    }

    // 8:13: 8:17 error: capture of moved value: `data` [E0382]
    // 8             data[i] += 1;
    //               ^~~~
    // 7:23: 9:10 note: `data` moved into closure environment here because it has type `collections::vec::Vec<i32>`, which is non-copyable
    // 7         thread::spawn(move || {
    // 8             data[i] += 1;
    // 9         });
    // 7:23: 9:10 help: perhaps you meant to use `clone()`?


    thread::sleep_ms(1000);
}
