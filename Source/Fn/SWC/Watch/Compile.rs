
/// Asynchronously compiles TypeScript files and outputs the results.
///
/// This function performs the following steps:
/// 1. Filters and processes the provided entries based on the given pattern and separator.
/// 2. Spawns asynchronous tasks to compile each TypeScript file.
/// 3. Collects the results and outputs them.
///
/// # Arguments
///
/// * `options` - A struct containing compilation options and file patterns.
///
/// # Example
///
/// ```rust
/// let options = CompilerOptions {
///     entry: vec![vec!["path".to_string(), "to".to_string(), "file.ts".to_string()]],
///     separator: '/',
///     pattern: ".ts".to_string(),
///     config: CompilerConfig::default(),
/// };
/// compile_typescript(options).await;
/// ```
///
/// # Errors
///
/// This function will return an error if it fails to compile files or send results.
#[instrument(skip(options))]
pub async fn Fn(options: CompilerOptions) -> Result<()> {
	let (tx, mut rx) = mpsc::unbounded_channel();
	let queue = FuturesUnordered::new();

	let compiler = Arc::new(Compiler::new(options.config.clone()));

	let files: Vec<String> = options
		.entry
		.into_par_iter()
		.filter_map(|entry| {
			entry
				.last()
				.filter(|last| last.ends_with(&options.pattern))
				.map(|_| entry[0..entry.len() - 1].join(&options.separator.to_string()))
		})
		.collect();

	for file in files {
		let tx = tx.clone();

		let compiler = Arc::clone(&compiler);

		queue.push(tokio::spawn(async move {
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
		queue.collect::<Vec<_>>().await;
		drop(tx);
	});

	let mut successful_compilations = 0;
	let mut failed_compilations = 0;

	while let Some((file, result)) = rx.recv().await {
		match result {
			Ok(output) => {
				info!("Compiled: {} -> {}", file, output);
				successful_compilations += 1;
			}
			Err(e) => {
				warn!("Failed to compile {}: {}", file, e);
				failed_compilations += 1;
			}
		}
	}

	let metrics = compiler.metrics.lock().await;
	info!(
		"Compilation complete. Processed {} files in {:?}. {} successful, {} failed.",
		metrics.files_processed, metrics.total_time, successful_compilations, failed_compilations
	);

	Ok(())
}
