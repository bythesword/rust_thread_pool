use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;
use thread_pool::ThreadPool;

fn main() {
    let (tx, rx) = mpsc::channel();
    let pool = ThreadPool::new(2);
    let mut i: i128 = 0;
    let main_receiver = receiver(rx);
    
    while true {
        let j = i.clone();
        let tx1 = tx.clone();
        let mut end = false;
        if i > 2 {
            end = true;
        }
        pool.execute(move || {
            handle_tx(j, end, tx1);
        });
        i += 1;
        thread::sleep(Duration::from_millis(1000));
        if i > 3{
            break;
        }
    }
    main_receiver.join().unwrap();
}