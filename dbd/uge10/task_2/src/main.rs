mod bloomfilter;
use bloomfilter::{BloomFilter};
use std::hash::{BuildHasher, Hash, Hasher};
use bit_vec::BitVec;

fn main() {
    let desired_capacity = 1_000_000;
    let desired_error_rate = 0.01;
    BloomFilter::<BitVec>::print_optimal_km(desired_capacity, desired_error_rate);
}
