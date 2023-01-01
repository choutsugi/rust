use rand::{self, RngCore};

pub fn add_one(value: usize) -> usize {
    println!("{}", rand::thread_rng().next_u64());

    value + 1
}
