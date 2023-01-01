use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// 为MyBox实现Deref特征
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));

    let _t1: &MyBox<String> = &m;
    let _t2: &String = _t1.deref(); // 解引用
    let _t3: &str = _t2.deref(); // 解引用

    hello(&(*m)[..]); // 手动解引用
    hello(&m); // 自动解引用：&MyBox<String> -> &String -> &str
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
