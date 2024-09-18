#[tracing::instrument(skip(Option))]
pub async fn Fn(Option: super::Option) -> Result<()> {
	let (tx, mut rx) = mpsc::unbounded_channel();
	let Queue = FuturesUnordered::new();

	let Compiler = Arc::new(crate::Struct::SWC::Compiler::new(Option.config.clone()));

	for file in Option
		.entry
		.into_par_iter()
		.filter_map(|entry| {
			entry
				.last()
				.filter(|last| last.ends_with(&Option.pattern))
				.map(|_| entry[0..entry.len() - 1].join(&Option.separator.to_string()))
		})
		.collect()
	{
		let tx = tx.clone();

		let compiler = Arc::clone(&Compiler);

		Queue.push(tokio::spawn(async move {
			match fs::read_to_string(&file).await {
				Ok(input) => match compiler.compile_file(&file, input).await {
					Ok(output) => {
						if let Err(e) = tx.send((file.clone(), Ok(output))) {
							error!("Cannot send compilation result: {}", e);
						}
					}
					Err(e) => {
						error!("Compilation error for {}: {}", file, e);
						if let Err(e) = tx.send((file.clone(), Err(e))) {
							error!("Cannot send compilation error: {}", e);
						}
					}
				},
				Err(e) => {
					error!("Failed to read file {}: {}", file, e);
					if let Err(e) = tx.send((file.clone(), Err(e.into()))) {
						error!("Cannot send file read error: {}", e);
					}
				}
			}
		}));
	}

	tokio::spawn(async move {
		Queue.collect::<Vec<_>>().await;
		drop(tx);
	});

	let mut Count = 0;
	let mut Error = 0;

	while let Some((file, result)) = rx.recv().await {
		match result {
			Ok(output) => {
				info!("Compiled: {} -> {}", file, output);
				Count += 1;
			}
			Err(e) => {
				warn!("Failed to compile {}: {}", file, e);
				Error += 1;
			}
		}
	}

	let Outlook = Compiler.metrics.lock().await;

	info!(
		"Compilation complete. Processed {} files in {:?}. {} successful, {} failed.",
		Outlook.files_processed, Outlook.total_time, Count, Error
	);

	Ok(())
}

use tracing::{error, info, warn};
