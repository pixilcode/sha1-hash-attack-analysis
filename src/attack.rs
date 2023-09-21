use std::collections::HashSet;

use crate::hash::sha1_hash;

// NOTE: start_value given so that we can have variety in results
pub fn collision_attack(guess_start_value: usize, bit_size: usize) -> usize {
    let mut generated_hashes: HashSet<Vec<u8>> = HashSet::new();
    let mut i: usize = guess_start_value;

    loop {
        let string_hash = sha1_hash(i.to_string(), bit_size);

        if generated_hashes.contains(&string_hash) {
            break i;
        }

        generated_hashes.insert(string_hash);
        i += 1;
    }
}

pub fn preimage_attack(digest: impl AsRef<[u8]>, bit_size: usize) -> usize {
    let digest = digest.as_ref();
    let mut i: usize = 0;

    loop {
        let string_hash = sha1_hash(i.to_string(), bit_size);

        if string_hash == digest {
            break i;
        }

        i += 1;
    }
}
