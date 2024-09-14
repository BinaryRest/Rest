/// Computes the hash of the given input using the `DefaultHasher`.
///
/// This function takes any input that implements the `Hash` trait and computes its hash value
/// using the `DefaultHasher` from the standard library. The resulting hash is returned as a `u64`.
///
/// # Arguments
///
/// * `Input` - A reference to the input value to be hashed. The input must implement the `Hash` trait.
///
/// # Returns
///
/// Returns a `u64` representing the hash value of the input.
///
/// # Example
///
/// ```rust
/// use std::collections::hash_map::DefaultHasher;
/// use std::hash::{Hash, Hasher};
///
/// let value = "example";
/// let hash = Fn(&value);
/// println!("Hash value: {}", hash);
/// ```
///
/// # Panics
///
/// This function does not panic.
///
/// # Errors
///
/// This function does not return errors.
pub fn Fn<T: Hash>(Input: &T) -> u64 {
	let mut Output = DefaultHasher::new();
	Input.hash(&mut Output);
	Output.finish()
}

use std::{
	collections::hash_map::DefaultHasher,
	hash::{Hash, Hasher},
};
