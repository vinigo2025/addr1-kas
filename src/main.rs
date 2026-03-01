use kaspa_keygen::*;
use std::sync::atomic::Ordering;
use std::thread;
use std::time::Duration;

fn main() {
    let plnum = thread::available_parallelism().expect("Sys err");
    println!("Working...");

    for _ in 0..plnum.get() {
        thread::spawn(entr);
    }

    loop {
        thread::sleep(Duration::from_millis(500));
        let cnt = CNT.load(Ordering::Relaxed);
        if cnt > 0 {
            break;
        }
    }
}
