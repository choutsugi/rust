pub fn add_two(value: i32) -> i32 {
    internal_adder(value, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

// 单元测试
#[cfg(test)]
mod tests {
    use super::*; // 引入父模块

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 4))
    }
}
