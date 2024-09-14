/// Processes and prints summaries of differences.
///
/// This function takes an iterator of summaries, processes them to aggregate differences
/// by their associated messages, and then prints the aggregated results. The summaries
/// are expected to be in the form of a tuple containing an entry string and a `DashMap`
/// of differences.
///
/// # Type Parameters
///
/// * `I` - An iterator type that yields items of type `(String, DashMap<u64, (String, String)>)`.
///
/// # Arguments
///
/// * `summaries` - An iterator of summaries, where each summary is a tuple containing:
///   - `Entry`: A `String` representing the entry associated with the summary.
///   - `Build`: A `DashMap<u64, (String, String)>` where the key is a hash and the value is a tuple
///     containing a difference string and a message string.
///
/// # Example
///
/// ```rust
/// use dashmap::DashMap;
/// use std::collections::HashSet;
/// use itertools::Itertools;
/// use std::cmp::Reverse;
///
/// let mut summary1 = DashMap::new();
/// summary1.insert(1, ("diff1".to_string(), "message1".to_string()));
///
/// let mut summary2 = DashMap::new();
/// summary2.insert(2, ("diff2".to_string(), "message2".to_string()));
///
/// let summaries = vec![
///     ("entry1".to_string(), summary1),
///     ("entry2".to_string(), summary2),
/// ];
///
/// Fn(summaries);
/// ```
///
/// # Panics
///
/// This function does not panic.
///
/// # Errors
///
/// This function does not return errors.
pub fn Fn<I>(Build: I)
where
	I: IntoIterator<Item = (String, DashMap<u64, (String, String)>)>,
{
	let Output: DashMap<String, HashSet<String>> = DashMap::new();

	for (Entry, Build) in Build {
		for (_, (Difference, Message)) in Build.into_iter() {
			Output
				.entry(Message + " in " + &Entry)
				.and_modify(|Existing: &mut HashSet<String>| {
					Existing.insert(Difference.clone());
				})
				.or_insert_with(|| {
					let mut New = HashSet::new();
					New.insert(Difference);
					New
				});
		}
	}

	Output.into_iter().sorted_by(|(A, _), (B, _)| A.cmp(B)).for_each(|(Message, Difference)| {
		println!("{}", Message);

		Difference
			.into_iter()
			.sorted_by_key(|Difference| Reverse(Difference.len()))
			.for_each(|Difference| println!("{}", Difference));
	});
}

use dashmap::DashMap;
use itertools::Itertools;
use std::{cmp::Reverse, collections::HashSet};
