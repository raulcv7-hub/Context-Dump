use ignore::DirEntry;

pub struct NoiseDetector;

impl NoiseDetector {
    /// Returns true if the entry should be strictly ignored based on static and dynamic rules.
    pub fn is_noise(entry: &ignore::DirEntry) -> bool {
        let name = entry.file_name().to_string_lossy().to_lowercase();

        const NOISE_DIRS: &[&str] = &[
            ".git",
            ".svn",
            ".hg",
            "node_modules",
            "target",
            "build",
            "dist",
            "__pycache__",
            ".venv",
            "venv",
            "env",
            ".idea",
            ".vscode",
            ".next",
            ".nuxt",
            ".turbo",
            ".yarn",
            "bower_components",
            "vendor",
            "coverage",
            "tmp",
            "out",
            "bin",
            "obj",
            ".terraform",
            ".serverless",
            ".aws",
        ];

        const NOISE_FILES: &[&str] = &[
            "cargo.lock",
            "package-lock.json",
            "yarn.lock",
            "pnpm-lock.yaml",
            "poetry.lock",
            "pipfile.lock",
            ".ds_store",
            "thumbs.db",
            "db.sqlite",
        ];

        if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
            return NOISE_DIRS.contains(&name.as_ref());
        }

        NOISE_FILES.contains(&name.as_ref()) || Self::is_heavy_artifact(entry)
    }

    /// Detects if a file is a binary artifact or too heavy to be useful context.
    fn is_heavy_artifact(entry: &DirEntry) -> bool {
        let path = entry.path();
        let ext = path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_lowercase();

        const FORBIDDEN_EXTS: &[&str] = &[
            "exe", "dll", "so", "dylib", "pyc", "wasm", "pdb", "zip", "tar", "gz", "7z", "png",
            "jpg", "jpeg", "gif", "svg", "ico", "mp4", "mov", "bin", "iso",
        ];

        if FORBIDDEN_EXTS.contains(&ext.as_str()) {
            return true;
        }

        let file_size = entry.metadata().map(|m| m.len()).unwrap_or(0);

        let limit = match ext.as_str() {
            "pdf" | "docx" | "xlsx" | "xls" => 1_000_000_000, // 1GB
            "xml" | "json" | "csv" | "sql" => 250_000_000,    // 250MB
            _ => 50_000_000,                                  // 50MB
        };

        file_size > limit
    }
}
