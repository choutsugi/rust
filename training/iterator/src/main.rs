struct _Shoe {
    size: u32,
    style: String,
}

fn _shoes_in_my_size(shoes: Vec<_Shoe>, shoe_size: u32) -> Vec<_Shoe> {
    // 通过迭代器过滤
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

fn main() {}
