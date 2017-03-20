// thread10.rs
use std::thread;
use std::sync::mpsc;


fn main() {
    let (tx, rx) = mpsc::sync_channel(0);
    let t1 = thread::spawn(move || {
        for i in 0..5 {
            tx.send(i).unwrap();
        }
    });

    for _ in 0..5 {
        let res = rx.recv().unwrap();
        println!("{}",res);
    }
    t1.join().unwrap();

}
