use rayon::prelude::*;

use crate::{attack::{collision_attack, preimage_attack}, hash::sha1_hash};

const BIT_SIZES: [usize; 6] = [8, 11, 16, 22, 27, 32];

const DEFAULT_SAMPLES: usize = 50;

pub struct BenchResult(usize);

pub fn run_collision_bench(bit_size: usize, num_samples: Option<usize>) -> Vec<BenchResult> {
	let num_samples = num_samples.unwrap_or(DEFAULT_SAMPLES);
	(0..num_samples).into_par_iter().map(|_| {
		let guess_start_value = rand::random();
		let result = collision_attack(guess_start_value, bit_size);
		BenchResult(result)
	}).collect()
}

pub fn run_preimage_bench(bit_size: usize, num_samples: Option<usize>) -> Vec<BenchResult> {
	let num_samples = num_samples.unwrap_or(DEFAULT_SAMPLES);
	(0..num_samples).into_par_iter().map(|_| {
		let preimage: [char; 16] = rand::random();
		let preimage: String = String::from_iter(preimage);
		let preimage_hash = sha1_hash(preimage, bit_size);
		let result = preimage_attack(preimage_hash, bit_size);
		BenchResult(result)
	}).collect()
}