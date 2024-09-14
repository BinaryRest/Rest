/// Inserts a difference summary into the provided DashMap.
///
/// This function computes the hash of the given difference string and inserts it into the DashMap
/// along with the associated message. The hash is used as the key, and the value is a tuple
/// containing the difference string and the message.
///
/// # Arguments
///
/// * `Build` - A reference to a DashMap where the difference summary will be inserted.
/// * `Difference` - A string representing the difference to be summarized.
/// * `Message` - A string representing the message associated with the difference.
///
/// # Example
///
/// ```rust
/// let summary = DashMap::new();
/// let difference = "diff content".to_string();
/// let message = "Build message".to_string();
/// Fn(&summary, difference, message);
/// ```
///
/// # Panics
///
/// This function does not panic.
///
/// # Errors
///
/// This function does not return errors.
pub fn Fn(Build: &DashMap<u64, (String, String)>, Difference: String, Message: String) {
	Build.insert(Hash::Fn(&Difference), (Difference, Message));
}

use dashmap::DashMap;

pub mod Hash;
