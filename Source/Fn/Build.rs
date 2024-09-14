pub async fn Fn(
	Entry: &str,
) -> Result<DashMap<u64, (String, String)>, Box<dyn std::error::Error>> {
	let Build = DashMap::new();

	match Repository::open(Entry) {
		Ok(Repository) => {
			let Name = Repository.tag_names(None)?;

			let mut Date: Vec<(String, DateTime<FixedOffset>)> = Name
				.iter()
				.filter_map(|Tag| {
					Tag.and_then(|Tag| {
						Repository
							.revparse_single(&Tag)
							.ok()
							.and_then(|Commit| Commit.peel_to_commit().ok())
							.map(|Commit| {
								(
									Tag.to_string(),
									DateTime::from_timestamp(Commit.time().seconds(), 0)
										.unwrap()
										.fixed_offset(),
								)
							})
					})
				})
				.collect();

			Date.sort_by(|A, B| A.1.cmp(&B.1));

			let Tag: Vec<String> = Date.into_iter().map(|(Tag, _)| Tag).collect();

			let Head = Repository.head()?;

			let First = Repository.find_commit(First::Fn(&Repository)?)?.id().to_string();

			let Last = Head.peel_to_commit()?.id().to_string();

			if Tag.is_empty() {
				Insert::Fn(
					&Build,
					crate::Fn::Build::Difference::Fn(&Repository, &First, &Last, Option)?,
					format!("⛱️ Build from first commit to last commit"),
				)
			} else {
				if let Some(Latest) = Tag.last() {
					Insert::Fn(
						&Build,
						crate::Fn::Build::Difference::Fn(&Repository, Latest, &Last, Option)?,
						format!("⛱️ Build from {} to last commit", Latest),
					);
				}

				for Window in Tag.windows(2) {
					let Start = &Window[0];
					let End = &Window[1];

					Insert::Fn(
						&Build,
						crate::Fn::Build::Difference::Fn(&Repository, &Start, &End, Option)?,
						format!("⛱️ Build from {} to {}", Start, End),
					);
				}
			}
		}
		Err(_Error) => {
			eprintln!("Cannot Repository: {}", _Error);

			return Err(_Error.into());
		}
	}

	Ok(Build)
}

use chrono::{DateTime, FixedOffset};
use dashmap::DashMap;
use git2::Repository;

pub mod Difference;
pub mod First;
pub mod Group;
pub mod Insert;
