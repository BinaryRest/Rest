/// Retrieves the OID of the first commit in the repository.
///
/// This function initializes a revwalk on the given repository, pushes the HEAD reference onto the
/// revwalk, and sets the sorting mode to topological and reverse. It then retrieves the first commit
/// in the revwalk, which corresponds to the first commit in the repository.
///
/// # Arguments
///
/// * `Repository` - A reference to the git repository.
///
/// # Returns
///
/// Returns a `Result` containing the OID of the first commit if successful, or a `git2::Error` if an error occurs.
///
/// # Errors
///
/// This function will return an error if there are issues with initializing the revwalk, pushing the HEAD reference,
/// setting the sorting mode, or retrieving the first commit.
///
/// # Example
///
/// ```rust
/// let repo = git2::Repository::open("/path/to/repo").expect("Cannot open repository.");
/// let first_commit_oid = Fn(&repo).expect("Cannot retrieve first commit.");
/// println!("First commit OID: {}", first_commit_oid);
/// ```
///
/// # Panics
///
/// This function does not panic.
pub fn Fn(Repository: &Repository) -> Result<Oid, git2::Error> {
	let mut Walk = Repository.revwalk()?;
	Walk.push_head()?;
	Walk.set_sorting(Sort::TOPOLOGICAL | Sort::REVERSE)?;

	match Walk.next() {
		Some(Ok(Identifier)) => Ok(Identifier),
		Some(Err(_Error)) => Err(_Error),
		None => Err(git2::Error::from_str("Cannot git2.")),
	}
}

use git2::{Oid, Repository, Sort};
