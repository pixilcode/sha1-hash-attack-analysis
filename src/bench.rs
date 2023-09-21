use rayon::prelude::*;
use std::fmt;

use crate::{attack::{collision_attack, preimage_attack}, hash::sha1_hash};

const BIT_SIZES: [usize; 6] = [8, 11, 16, 22, 27, 32];

#[derive(Debug)]
pub struct BenchResult(usize);

impl fmt::Display for BenchResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.0.fmt(f)
    }
}

pub fn run_collision_bench(bit_size: usize, num_samples: usize) -> Vec<BenchResult> {
	(0..num_samples).into_iter().map(|i| {
		let guess_start_value = rand::random();
				let result = collision_attack(guess_start_value, bit_size);
				BenchResult(result)
	}).collect()
}

pub fn run_preimage_bench(bit_size: usize, num_samples: usize) -> Vec<BenchResult> {
	(0..num_samples).into_par_iter().map(|i| {
		let preimage: [char; 16] = rand::random();
		let preimage: String = String::from_iter(preimage);
		let preimage_hash = sha1_hash(preimage, bit_size);
		let result = preimage_attack(preimage_hash, bit_size);
				BenchResult(result)
	}).collect()
}