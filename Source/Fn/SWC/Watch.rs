
#[instrument]
pub async fn Fn(path: PathBuf, options: CompilerOptions) -> Result<()> {
	let (tx, mut rx) = mpsc::unbounded_channel();

	let mut watcher = RecommendedWatcher::new(
		move |res| {
			let _ = futures::executor::block_on(async {
				tx.send(res).unwrap();
			});
		},
		Config::default(),
	)?;

	watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

	while let Some(res) = rx.recv().await {
		match res {
			Ok(event) => {
				if let notify::Event {
					kind: notify::EventKind::Modify(notify::event::ModifyKind::Data(_)),
					paths,
					..
				} = event
				{
					for path in paths {
						if path.extension().map_or(false, |ext| ext == "ts") {
							let file_options = CompilerOptions {
								entry: vec![vec![path.to_string_lossy().to_string()]],
								..options.clone()
							};
							task::spawn(async move {
								if let Err(e) = Compile::Fn(file_options).await {
									error!("Compilation error: {}", e);
								}
							});
						}
					}
				}
			}
			Err(e) => error!("Watch error: {:?}", e),
		}
	}

	Ok(())
}

pub mod Compile;
