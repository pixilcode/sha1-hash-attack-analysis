use rayon::prelude::*;

use crate::{
    attack::{collision_attack, preimage_attack},
    hash::sha1_hash,
};

#[derive(Debug, PartialEq)]
pub struct BenchResult {
	pub bit_size: usize,
	pub num_hashes: usize,
}

impl PartialOrd for BenchResult {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.bit_size == other.bit_size {
            self.num_hashes.partial_cmp(&other.num_hashes)
        } else {
            self.bit_size.partial_cmp(&other.bit_size)
        }
    }
}

pub fn run_collision_bench(bit_size: usize, num_samples: usize) -> Vec<BenchResult> {
    (0..num_samples)
        .into_par_iter()
        .map(|_| {
            let guess_start_value = rand::random();
            let result = collision_attack(guess_start_value, bit_size);
            BenchResult { bit_size, num_hashes: result }
        })
        .collect()
}

pub fn run_preimage_bench(bit_size: usize, num_samples: usize) -> Vec<BenchResult> {
    (0..num_samples)
        .into_par_iter()
        .map(|_| {
            let preimage: [char; 16] = rand::random();
            let preimage: String = String::from_iter(preimage);
            let preimage_hash = sha1_hash(preimage, bit_size);
            let result = preimage_attack(preimage_hash, bit_size);
            BenchResult { bit_size, num_hashes: result }
        })
        .collect()
}
