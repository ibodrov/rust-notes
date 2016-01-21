use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    let data = Arc::new(Mutex::new(vec![0, 1, 2]));

    for i in 0..3 {
        let y = data.clone();
        thread::spawn(move || {
            let mut data = y.lock().unwrap();
            data[i] += 1;
        });
    }

    thread::sleep_ms(1000);
}
