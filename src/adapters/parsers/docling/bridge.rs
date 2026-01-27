use crate::adapters::parsers::docling::dto::DoclingResult;
use crate::adapters::parsers::docling::resolver::PythonResolver;
use anyhow::{anyhow, Context, Result};
use std::path::Path;
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};

static ENGINE_BROKEN: AtomicBool = AtomicBool::new(false);

pub struct DoclingBridge;

impl DoclingBridge {
    pub fn run(path: &Path) -> Result<DoclingResult> {
        if ENGINE_BROKEN.load(Ordering::Relaxed) {
            return Err(anyhow!(
                "Motor de IA desactivado por fallos de entorno previos."
            ));
        }

        let py_exec = PythonResolver::resolve();

        let abs_path = std::fs::canonicalize(path)
            .with_context(|| format!("Archivo no accesible: {:?}", path))?;
        let path_str = abs_path.to_str().context("Error de encoding en ruta")?;

        let script = r#"
import sys, json, os, logging
os.environ["TF_CPP_MIN_LOG_LEVEL"] = "3"
logging.getLogger("docling").setLevel(logging.ERROR)
try:
    import tiktoken
    from docling.document_converter import DocumentConverter, PdfFormatOption
    from docling.datamodel.pipeline_options import PdfPipelineOptions
    from docling.datamodel.base_models import InputFormat
except ImportError as e:
    print(json.dumps({"error_type": "DEPENDENCY_MISSING", "message": str(e)}))
    sys.exit(2)

try:
    pipeline_options = PdfPipelineOptions()
    pipeline_options.do_ocr = False
    pipeline_options.do_table_structure = False
    
    conv = DocumentConverter(
        format_options={InputFormat.PDF: PdfFormatOption(pipeline_options=pipeline_options)}
    )
    res = conv.convert(sys.argv[1])
    md = res.document.export_to_markdown()
    enc = tiktoken.get_encoding("cl100k_base")
    print(json.dumps({"content": md, "token_count": len(enc.encode(md))}))
    sys.exit(0)
except Exception as e:
    print(json.dumps({"error_type": "RUNTIME_ERROR", "message": str(e)}))
    sys.exit(1)
"#;

        let output = Command::new(&py_exec)
            .arg("-c")
            .arg(script)
            .arg(path_str)
            .output()
            .map_err(|e| anyhow!("Error al invocar Python ({:?}): {}", py_exec, e))?;

        let stdout = String::from_utf8_lossy(&output.stdout);

        if let Ok(val) = serde_json::from_str::<serde_json::Value>(&stdout) {
            if let Some(err_type) = val.get("error_type") {
                if err_type == "DEPENDENCY_MISSING" {
                    ENGINE_BROKEN.store(true, Ordering::SeqCst);
                }
                return Err(anyhow!("Python Fail [{}]: {}", err_type, val["message"]));
            }
            Ok(serde_json::from_value(val)?)
        } else {
            Err(anyhow!("Fallo crítico en subproceso Python (Docling)"))
        }
    }
}
