use std::{rc::Rc, sync::Arc, sync::Mutex, thread};

fn main() {
    // let counter = Rc::new(Mutex::new(0));
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        // let counter = Rc::clone(&counter); // 引用计数器+1，（非线程安全）
        let counter = Arc::clone(&counter); // Arc：原子级别引用计数器（线程安全）

        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
