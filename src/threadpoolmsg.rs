use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;
use crate::ThreadPool;

pub fn receiver(rx: Receiver<(i128, bool)>) -> thread::JoinHandle<()> {
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