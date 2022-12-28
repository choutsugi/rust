struct CustomSmartPointer {
    data: String,
}

// Drop 自动调用
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointer created.");
    println!("c.data = {}, d.data = {}", c.data, d.data);

    drop(c); // 手动销毁
    drop(d); // 手动销毁
}
