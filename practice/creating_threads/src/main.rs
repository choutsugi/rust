use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    // 移动vec所有权到子线程
    let handler = thread::spawn(move || {
        println!("Here's a vector: {:?}!", v);
    });

    handler.join().unwrap() // 阻塞主线程等待子线程执行结束
}
