use bit_vec::BitVec;
use fasthash::{murmur3, Murmur3HasherExt};
use indoc::indoc;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

// To specify precision rate.
// m - The amount of bits allocated
// k - number of functions used for hashing
// Notes: Don't use cryptographic hash functions - Too expensive.
// Common hash function is murmur. External crate.

// We can create a bloom filter where we specify the capacity and number of functions we want to parse through.
// In this case the failure rate will be unknown.

// We can also create a bloom filter where we specify the capacity and the success rate.
// Here the amount of functions we need to use will be unknown.

// It's math functions basically.

// Using some logic from here: https://sagi.io/bloom-filters-for-the-perplexed/#appendix
// Finding this a bit hard to wrap my head around...

pub struct BloomFilter<T: ?Sized> {
    bitmap: BitVec,
    optimal_k: u32,
    optimal_m: u64,
    _marker: PhantomData<T>,
}

impl<T: ?Sized> BloomFilter<T> {
    pub fn new(capacity: u64, error_rate: f64) -> Self {
        let optimal_km = Self::calculate_optimal_km(capacity, error_rate);
        let optimal_k = optimal_km.0;
        let optimal_m = optimal_km.1;
        BloomFilter {
            bitmap: BitVec::from_elem(optimal_m as usize, false),
            optimal_k,
            optimal_m,
            _marker: PhantomData,
        }
    }

    pub fn add(&mut self, item: &T)
    where
        T: Hash,
    {
        let (h1, h2) = self.hash_kernel(item);

        for i in 0..self.optimal_k {
            let index = self.get_index(h1, h2, i as u64);
            self.bitmap.set(index, true);
        }
    }

    pub fn check(&mut self, item: &T) -> bool
    where
        T: Hash,
    {
        let (h1, h2) = self.hash_kernel(item);

        for i in 0..self.optimal_k {
            let index = self.get_index(h1, h2, i as u64);

            if !self.bitmap.get(index).unwrap() {
                return false;
            }
        }
        true
    }

    fn hash_kernel(&self, item: &T) -> (u64, u64)
    where
        T: Hash,
    {
        let mut s: Murmur3HasherExt = Default::default();
        item.hash(&mut s);
        let hash1 = s.finish();
        item.hash(&mut s);
        let hash2 = s.finish();
        (hash1, hash2)
    }

    fn get_index(&self, h1: u64, h2: u64, i: u64) -> usize {
        (h1.wrapping_add((i).wrapping_mul(h2)) % self.optimal_m) as usize
    }

    pub fn calculate_optimal_km(capacity: u64, error_rate: f64) -> (u32, u64) {
        let two_point_zero: f64 = 2.0;
        let ln2 = two_point_zero.ln();
        let lnp = error_rate.ln();
        let k = -lnp / ln2;
        let m = -(capacity as i64) as f64 * lnp / (f64::powi(ln2, 2));
        return (f64::ceil(k) as u32, f64::ceil(m) as u64);
    }

    pub fn print_optimal_km(capacity: u64, error_rate: f64) {
        let optimal_km = Self::calculate_optimal_km(capacity, error_rate);
        println!(
            indoc! {
            "

            The desired capacity was: {} 
            The desired error rate was: {} 
            The required hash function call count was: {} 
            and {} bits per element
            "},
            capacity,
            error_rate,
            optimal_km.0,
            optimal_km.1 as f64 / capacity as f64
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let mut bloom = BloomFilter::new(100, 0.01);
        bloom.add("item");
        assert!(bloom.check("item"));
    }

    #[test]
    fn check_and_add() {
        let mut bloom = BloomFilter::new(100, 0.01);
        assert!(!bloom.check("item_1"));
        assert!(!bloom.check("item_2"));
        bloom.add("item_1");
        assert!(bloom.check("item_1"));
    }
}
