pub async fn Fn(Entry: &str) -> Result<DashMap<u64, (String, String)>, Box<dyn std::error::Error>> {
	let Build = DashMap::new();

	Ok(Build)
}

use chrono::{DateTime, FixedOffset};
use dashmap::DashMap;
use git2::Repository;

pub mod Difference;
pub mod First;
pub mod Group;
pub mod Insert;
