use List::{Cons, Nil};

enum List {
    Cons(i32, Box<List>), // 使用智能指针包裹（固定大小）
    Nil,
}

fn main() {
    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
