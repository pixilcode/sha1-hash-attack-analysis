use sha1::{Digest, Sha1};

pub fn sha1_hash(input: impl AsRef<str>, bit_size: usize) -> u32 {
    assert!(bit_size < 32);

    let mut hasher = Sha1::new();
    hasher.update(input.as_ref());
    let result = hasher.finalize();
    let result = u32::from_be_bytes([result[0], result[1], result[2], result[3]]);
    let bit_mask = (1u32 << bit_size).wrapping_sub(1);
    result & bit_mask
}
