/**
 * 创建闭包时：编译器根据闭包中对值的使用方式自动推导所需使用的函数特征。
 *  FnOnce：获取作用域中变量的所有权。
 *  FnMut：以可变形式借用变量。
 *  Fn：以不可变形式借用变量。
 */
fn main() {
    let x = vec![1, 2, 3];

    let equal_to_x = move |z: Vec<i32>| z == x; // 定义闭包：闭包访问外部变量x

    // println!("{:#?}", x); // err: borrow after move

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}
