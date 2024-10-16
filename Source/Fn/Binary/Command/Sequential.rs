/// Asynchronously processes entries to generate summaries and outputs the
/// results sequentially.
///
/// This function performs the following steps:
/// 1. Filters and processes the provided entries based on the given pattern and
///    separator.
/// 2. Spawns asynchronous tasks to generate summaries for each entry.
/// 3. Collects the results and outputs them.
///
/// # Arguments
///
/// * `Option` - A struct containing the following fields:
///   - `Entry`: A vector of vectors, where each inner vector contains the
///     components of a file path.
///   - `Separator`: A character used to join the components of the file path.
///   - `Pattern`: A string pattern to match against the last element of each
///     entry.
///
/// # Example
///
/// ```rust
/// let options = Option {
/// 	Entry:vec![vec!["path".to_string(), "to".to_string(), "file.git".to_string()]],
/// 	Separator:'/',
/// 	Pattern:".git".to_string(),
/// };
/// Fn(options).await;
/// ```
///
/// # Errors
///
/// This function will log errors if it fails to generate summaries or send
/// results.
pub async fn Fn(Option { Entry, Pattern, Separator, .. }:Option) {
	let Queue = futures::future::join_all(
		Entry
			.into_iter()
			.filter_map(|Entry| {
				Entry
					.last()
					.filter(|Last| *Last == &Pattern)
					.map(|_| Entry[0..Entry.len() - 1].join(&Separator.to_string()))
			})
			.map(|Entry| {
				async move {
					match crate::Fn::Build::Fn(&Entry).await {
						Ok(Build) => Ok((Entry, Build)),
						Err(_Error) => {
							Err(format!("Error generating summary for {}: {}", Entry, _Error))
						},
					}
				}
			}),
	)
	.await;

	crate::Fn::Build::Group::Fn(Queue.into_iter().filter_map(Result::ok).collect::<Vec<_>>());
}

use crate::Struct::Binary::Command::Entry::Struct as Option;
