use std::path::PathBuf;
use std::sync::OnceLock;
use tracing::warn;

static CACHED_PYTHON: OnceLock<PathBuf> = OnceLock::new();

pub struct PythonResolver;

impl PythonResolver {
    /// Resuelve el intérprete de Python una sola vez y cachea el resultado.
    pub fn resolve() -> PathBuf {
        CACHED_PYTHON.get_or_init(Self::perform_resolution).clone()
    }

    fn perform_resolution() -> PathBuf {
        if let Ok(path) = std::env::var("CONTEXT_PYTHON") {
            return PathBuf::from(path);
        }

        let subpath = if cfg!(target_os = "windows") {
            "Scripts/python.exe"
        } else {
            "bin/python3"
        };

        if let Ok(cwd) = std::env::current_dir() {
            let p = cwd.join(".venv").join(subpath);
            if p.exists() {
                return p;
            }
        }

        if let Some(mut p) = dirs::config_dir() {
            p.push("context");
            p.push("venv");
            p.push(subpath);
            if p.exists() {
                return p;
            }
        }

        if let Ok(exe_path) = std::env::current_exe() {
            let mut current = exe_path.parent();
            while let Some(dir) = current {
                let venv_path = dir.join(".venv").join(subpath);
                if venv_path.exists() {
                    return venv_path;
                }
                current = dir.parent();
            }
        }

        warn!("No se encontró un entorno virtual (.venv) válido. Usando Python del sistema.");
        PathBuf::from("python3")
    }
}
