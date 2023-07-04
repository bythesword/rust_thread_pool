use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;
use tom_thread_pool::ThreadPool;

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
    // while true {
    //     let j = i.clone();
    //     pool.execute(move || {
    //         handle_connection(j);
    //     });
    //     i += 1;
    //     thread::sleep(Duration::from_millis(1000));
    // }
}

fn handle_connection(i: i128) {
    //print!(" handle called by {i} ;                          ");
    thread::sleep(Duration::from_millis(1500));
}
fn handle_tx(i: i128, end: bool, tx: Sender<(i128, bool)>) {
    //print!(" handle called by {i} ;                          ");
    tx.send((i, end)).unwrap();
    thread::sleep(Duration::from_millis(1500));
}
fn receiver(rx: Receiver<(i128, bool)>) -> thread::JoinHandle<()> {
    let main_receiver = thread::spawn(move || {
        loop {
            let mut flag = false;
            match rx.try_recv() {
                Ok((i, end)) => {
                    if end == true {
                        flag = end;
                        print!("the flag of end threed pool received . \n            ");
                        
                    } else {
                        print!(" receiver : received {i}.                               ");
                    }
                }
                _ => {
                    // println!("sub thread recv nothing");
                    // thread::sleep(Duration::from_secs_f32(0.5))
                }
            }
            if flag{
                break;
            }
        }
    });
    main_receiver
}
