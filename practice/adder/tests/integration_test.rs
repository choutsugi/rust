use adder;

// 声明模块
mod common;

#[test]
fn it_adds_two() {
    common::setup(); // 执行集成测试前置工作

    assert_eq!(4, adder::add_two(2));
}
