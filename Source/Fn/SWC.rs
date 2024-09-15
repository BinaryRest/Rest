use std::{
	path::{Path, PathBuf},
	sync::Arc,
	time::{Duration, Instant, SystemTime},
};

use anyhow::{Context, Result};
use futures::stream::{FuturesUnordered, StreamExt};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use tokio::{
	fs,
	sync::{mpsc, Mutex},
	task,
};
use tracing::{debug, error, info, instrument, warn};

use swc_common::{FileName, FilePathMapping, SourceMap};
use swc_ecma_ast::EsVersion;
use swc_ecma_codegen::{text_writer::JsWriter, Emitter};
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax, TsConfig};
use swc_ecma_transforms_base::{helpers::inject_helpers, resolver};
use swc_ecma_transforms_proposal::decorators;
use swc_ecma_transforms_typescript::strip;
use swc_ecma_visit::FoldWith;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FileInfo {
	path: PathBuf,
	last_modified: SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CompilerConfig {
	target: String,
	module: String,
	strict: bool,
	emit_decorators_metadata: bool,
}

#[derive(Debug, Clone)]
struct CompilerOptions {
	entry: Vec<Vec<String>>,
	separator: char,
	pattern: String,
	config: CompilerConfig,
}

#[derive(Debug, Default)]
struct CompilerMetrics {
	files_processed: usize,
	total_time: Duration,
	errors: usize,
}

impl Default for CompilerConfig {
	fn default() -> Self {
		Self {
			target: "es2022".to_string(),
			module: "commonjs".to_string(),
			strict: true,
			emit_decorators_metadata: true,
		}
	}
}

#[derive(Debug)]
struct Compiler {
	config: CompilerConfig,
	metrics: Arc<Mutex<CompilerMetrics>>,
}

impl Compiler {
	fn new(config: CompilerConfig) -> Self {
		Self { config, metrics: Arc::new(Mutex::new(CompilerMetrics::default())) }
	}

	#[instrument(skip(self, input))]
	async fn compile_file(&self, file: &str, input: String) -> Result<String> {
		let start_time = Instant::now();

		let cm = SourceMap::new(FilePathMapping::empty());

		let source_file = cm.new_source_file(FileName::Real(file.into()), input);

		let mut parser = Parser::new_from(Lexer::new(
			Syntax::Typescript(TsConfig { decorators: true, ..Default::default() }),
			EsVersion::Es2022,
			StringInput::from(&*source_file),
			None,
		));

		let mut module = parser.parse_module().expect("Failed to parse TypeScript module")?;

		let unresolved_mark = swc_common::DUMMY_SP.apply_mark(swc_common::Mark::new());

		let top_level_mark = swc_common::DUMMY_SP.apply_mark(swc_common::Mark::new());

		module = module.fold_with(&mut resolver(unresolved_mark, top_level_mark, true));
		module = module.fold_with(&mut strip());
		module = module.fold_with(&mut decorators::decorators(decorators::Config {
			legacy: false,
			emit_metadata: self.config.emit_decorators_metadata,
			use_define_for_class_fields: true,
			..Default::default()
		}));
		module = module.fold_with(&mut InjectHelpers::default());

		let mut buf = vec![];

		let mut emitter = Emitter {
			cfg: swc_ecma_codegen::Config::default(),
			cm: cm.clone(),
			comments: None,
			wr: JsWriter::new(cm, "\n", &mut buf, None),
		};

		emitter.emit_module(&module).context("Failed to emit JavaScript")?;

		let js_path = Path::new(file).with_extension("js");
		fs::write(&js_path, &buf).await.context("Failed to write output file")?;

		let elapsed = start_time.elapsed();

		let mut metrics = self.metrics.lock().await;
		metrics.files_processed += 1;
		metrics.total_time += elapsed;

		debug!("Compiled {} in {:?}", file, elapsed);

		Ok(js_path.to_string_lossy().to_string())
	}
}

#[tokio::main]
async fn main() -> Result<()> {
	tracing_subscriber::fmt::init();

	let args: Vec<String> = std::env::args().collect();
	if args.len() != 2 {
		error!("Usage: {} <directory>", args[0]);
		std::process::exit(1);
	}

	let Path = PathBuf::from(&args[1]);

	let config = if let Ok(config_file) = fs::read_to_string("swc_config.json").await {
		serde_json::from_str(&config_file).unwrap_or_default()
	} else {
		CompilerConfig::default()
	};

	let options = CompilerOptions {
		entry: vec![vec![Path.to_string_lossy().to_string()]],
		separator: std::path::MAIN_SEPARATOR,
		pattern: ".ts".to_string(),
		config: config.clone(),
	};

	// Initial compilation
	info!("Starting initial compilation...");
	compile_typescript(options.clone()).await?;

	info!("Initial compilation complete. Watching for changes...");

	// Start watching for changes
	Watch::Fn(Path, options).await?;

	Ok(())
}

pub mod Watch;
