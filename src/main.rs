use kaspa_keygen::*;
use std::sync::atomic::Ordering;
use std::thread;
use std::time::Duration;

fn main() {
    let plnum = thread::available_parallelism().expect("Sys err");
    let pnum = plnum.get();
    println!("Working...");

    loop {
        thread::sleep(Duration::from_millis(500));
        let cnt = CNT.load(Ordering::Relaxed);
        if cnt < pnum {
            let _ = thread::spawn(entr);
        }
    }
}
