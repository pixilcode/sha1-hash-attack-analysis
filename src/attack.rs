use std::collections::HashMap;

use crate::hash::sha1_hash;

#[derive(Debug, PartialEq)]
pub struct AttackResult {
    pub bit_size: usize,
    pub num_hashes: usize,
    pub collision_values: (String, String),
}

impl PartialOrd for AttackResult {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.bit_size == other.bit_size {
            self.num_hashes.partial_cmp(&other.num_hashes)
        } else {
            self.bit_size.partial_cmp(&other.bit_size)
        }
    }
}

// NOTE: start_value given so that we can have variety in results
pub fn collision_attack(guess_start_value: usize, bit_size: usize) -> AttackResult {
    let mut generated_hashes: HashMap<u32, usize> = HashMap::new();
    let mut i: usize = guess_start_value;

    loop {
        let string_hash = sha1_hash(i.to_string(), bit_size);

        if generated_hashes.contains_key(&string_hash) {
            let num_hashes = i - guess_start_value + 1;
            let collision_value_a = generated_hashes.get(&string_hash).unwrap().to_string();
            let collision_value_b = i.to_string();
            break AttackResult {
                bit_size,
                num_hashes,
                collision_values: (collision_value_a, collision_value_b),
            };
        }

        generated_hashes.insert(string_hash, i);
        i += 1;
    }
}

pub fn preimage_attack(initial_value: String, bit_size: usize) -> AttackResult {
    let digest = sha1_hash(&initial_value, bit_size);
    let mut i: usize = 0;

    loop {
        let string_hash = sha1_hash(i.to_string(), bit_size);

        if string_hash == digest {
            break AttackResult {
                bit_size,
                num_hashes: i,
                collision_values: (initial_value, i.to_string()),
            };
        }

        i += 1;
    }
}
