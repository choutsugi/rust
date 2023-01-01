use std::{sync::mpsc, thread, time::Duration};

fn main() {
    // mpsc: multi-producer, single-consumer
    let (tx1, rx) = mpsc::channel();

    let tx2 = tx1.clone();

    thread::spawn(move || {
        let vals = vec![String::from("i"), String::from("see"), String::from("u")];

        for val in vals {
            tx1.send(val).unwrap(); // 移交所有权
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![String::from("just"), String::from("4"), String::from("u")];

        for val in vals {
            tx2.send(val).unwrap(); // 移交所有权
            thread::sleep(Duration::from_secs(1));
        }
    });

    // let received = rx.try_recv().unwrap(); // 非阻塞接收
    // let received = rx.recv().unwrap(); // 阻塞接收
    for received in rx {
        println!("Got: {}", received);
    }
}
