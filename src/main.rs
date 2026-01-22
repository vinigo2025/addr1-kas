use kaspa_keygen::*;
use std::thread;
use std::time::Duration;
use std::sync::atomic::Ordering;

fn main() {
    println!("Working...");

    thread::spawn(|| entr());
    thread::spawn(|| entr());
    thread::spawn(|| entr());
    thread::spawn(|| entr());

    loop {
        thread::sleep(Duration::from_millis(500));
        let cnt = CNT.fetch_add(0, Ordering::SeqCst);
        if cnt > 0 { break; }
    }
}
