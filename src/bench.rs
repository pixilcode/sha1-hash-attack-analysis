use rayon::prelude::*;

use crate::attack::{collision_attack, preimage_attack, AttackResult};

pub fn run_collision_bench(bit_size: usize, num_samples: usize) -> Vec<AttackResult> {
    (0..num_samples)
        .into_par_iter()
        .map(|_| {
            let guess_start_value = rand::random();
            collision_attack(guess_start_value, bit_size)
        })
        .collect()
}

pub fn run_preimage_bench(bit_size: usize, num_samples: usize) -> Vec<AttackResult> {
    (0..num_samples)
        .into_par_iter()
        .map(|_| {
            let preimage: [char; 16] = rand::random();
            let preimage: String = String::from_iter(preimage);
            preimage_attack(preimage, bit_size)
        })
        .collect()
}
