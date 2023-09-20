use sha1::{Sha1, Digest};

pub fn sha1_hash(input: impl AsRef<str>, hash_length: usize) -> Vec<u8> {
	let mut hasher = Sha1::new();
	hasher.update(input.as_ref());
	let result = hasher.finalize();

	result.iter().take(hash_length).copied().collect()
}