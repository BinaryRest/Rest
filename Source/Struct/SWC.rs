#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
	path: PathBuf,
	last_modified: SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilerConfig {
	Target: String,
	Module: String,
	strict: bool,
	emit_decorators_metadata: bool,
}

#[derive(Debug, Clone)]
pub struct Option {
	entry: Vec<Vec<String>>,
	separator: char,
	pattern: String,
	config: CompilerConfig,
}

#[derive(Debug, Default)]
pub struct CompilerMetrics {
	Count: usize,
	Elapsed: Duration,
	Error: usize,
}

impl Default for CompilerConfig {
	fn default() -> Self {
		Self {
			Target: "es2022".to_string(),
			Module: "commonjs".to_string(),
			strict: true,
			emit_decorators_metadata: true,
		}
	}
}

#[derive(Debug)]
pub struct Compiler {
	config: CompilerConfig,
	Outlook: Arc<Mutex<CompilerMetrics>>,
}

impl Compiler {
	fn new(config: CompilerConfig) -> Self {
		Self { config, Outlook: Arc::new(Mutex::new(CompilerMetrics::default())) }
	}

	#[tracing::instrument(skip(self, input))]
	async fn compile_file(&self, file: &str, input: String) -> Result<String> {
		let Begin = Instant::now();

		let cm = SourceMap::new(FilePathMapping::empty());

		let source_file = cm.new_source_file(FileName::Real(file.into()), input);

		let mut parser = Parser::new_from(Lexer::new(
			Syntax::Typescript(TsConfig { decorators: true, ..Default::default() }),
			EsVersion::Es2022,
			StringInput::from(&*source_file),
			None,
		));

		let mut File = parser.parse_module().expect("Failed to parse TypeScript module")?;

		File =
			File.fold_with(&mut swc_ecma_transforms_base::resolver(Mark::new(), Mark::new(), true));
		File = File.fold_with(&mut swc_ecma_transforms_typescript::strip());
		File = File.fold_with(&mut decorators::decorators(decorators::Config {
			legacy: false,
			emit_metadata: self.config.emit_decorators_metadata,
			use_define_for_class_fields: true,
			..Default::default()
		}));
		File = File.fold_with(&mut InjectHelpers::default());

		let mut Output = vec![];

		let mut Emitter = Emitter {
			cfg: swc_ecma_codegen::Config::default(),
			cm: cm.into().clone(),
			comments: None,
			wr: JsWriter::new(cm.into(), "\n", &mut Output, None),
		};

		Emitter.emit_module(&File).context("Failed to emit JavaScript")?;

		let js_path = Path::new(file).with_extension("js");

		fs::write(&js_path, &Output).await.context("Failed to write output file")?;

		let Elapsed = Begin.elapsed();

		let mut Outlook = self.Outlook.lock().await;
		Outlook.Count += 1;
		Outlook.Elapsed += Elapsed;

		debug!("Compiled {} in {:?}", file, Elapsed);

		Ok(js_path.to_string_lossy().to_string())
	}
}

use serde::{Deserialize, Serialize};
use swc_common::DUMMY_SP;
use tracing::debug;
