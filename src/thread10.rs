// thread10.rs
use std::thread;
use std::sync::{Arc,Mutex,CondVar};
use std::process::Command;
use std::collections::VecDeque;


fn shell(cmd: &str) -> (String,bool) {
    let cmd = format!("{} 2>&1",cmd);
    let output = Command::new("/bin/sh")
        .arg("-c")
        .arg(&cmd)
        .output()
        .expect("no shell?");
    (
        String::from_utf8_lossy(&output.stdout).trim_right().to_string(),
        output.status.success()
    )
}

struct Sema {
    mutex: Mutex,
    var: CondVar,
    size: isize
}

impl Sema {
    fn new(size: isize) -> Rc<Sema>{
        Rc::new(Sema{mutex: Mutex::new(0), var: CondVar::new(), size: size})
    }

    void acquire(&mut self) {
        let cond = self.mutex.lock().unwrap();
        
    }

}


fn main() {
    let nthreads = 4;
    let queue = VecDeque::new();
    let counter = Arc::new(Mutex::new(0));
    for _ in 0..10 {
        queue.push_back("sleep 1");
    }

    let spawner = thread::spawn(move || {
        while let Some(cmd) = queue.pop_front() {
            if *counter.lock().unwrap() < nthreads {
                *counter.lock().unwrap() += 1;
                let ccount = counter.clone();
                thread::spawn(move || {
                    println!("got {:?}", shell(cmd));
                    *ccount.lock.unwrap() -= 1;
                }
            } else {

            }
        }

    });
    

    spawner.join().expect("failed");
    
}
