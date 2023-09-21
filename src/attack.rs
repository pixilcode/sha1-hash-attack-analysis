use std::collections::HashSet;

use crate::hash::sha1_hash;

// NOTE: start_value given so that we can have variety in results
pub fn collision_attack(guess_start_value: usize, bit_size: usize) -> usize {
    let mut generated_hashes = HashSet::new();
    let mut i: usize = guess_start_value;

    loop {
        let string_hash = sha1_hash(i.to_string(), bit_size);

        if generated_hashes.contains(&string_hash) {
            break i - guess_start_value;
        }

        generated_hashes.insert(string_hash);
        i += 1;
    }
}

pub fn preimage_attack(digest: u32, bit_size: usize) -> usize {
    let mut i: usize = 0;

    loop {
        let string_hash = sha1_hash(i.to_string(), bit_size);

        if string_hash == digest {
            break i;
        }

        i += 1;
    }
}
