use kaspa_keygen::*;
use std::thread;
use std::time::Duration;
use std::sync::atomic::Ordering;

fn main() {
    let plnum = thread::available_parallelism().expect("Sys err");
    println!("Working...");

    for _i in 0..plnum.get() {
        thread::spawn(|| entr());
    }

    loop {
        thread::sleep(Duration::from_millis(500));
        let cnt = CNT.load(Ordering::Relaxed);
        if cnt > 0 { break; }
    }
}
