use ignore::DirEntry;

/// Handles the detection of "noise" files and directories that should be ignored.
pub struct NoiseDetector;

impl NoiseDetector {
    /// Returns true if the entry is considered noise based on multiple criteria.
    pub fn is_noise(entry: &ignore::DirEntry) -> bool {
        let name = entry.file_name().to_string_lossy().to_lowercase();

        if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
            return Self::is_ignored_directory(&name);
        }

        Self::is_ignored_file(&name) || Self::is_heavy_artifact(entry)
    }

    /// Checks if a directory name matches common build, environment, or system folders.
    fn is_ignored_directory(name: &str) -> bool {
        const DIRS: &[&str] = &[
            ".git", ".svn", ".hg", "node_modules", "target", "build", "dist", "__pycache__",
            ".venv", "venv", "env", ".idea", ".vscode", ".next", ".nuxt", ".turbo", ".yarn",
            "bower_components", "vendor", "coverage", "tmp", "out", "bin", "obj", ".terraform",
            ".serverless", ".aws", ".docker", ".k8s", ".expo", ".gradle", ".dart_tool", ".infrastructure",
            "platforms", "plugins", "Pods", ".docz", ".cache", ".pytest_cache", "site-packages",
        ];
        DIRS.contains(&name)
    }

    /// Checks if a file name matches common lock files, logs, or system artifacts.
    fn is_ignored_file(name: &str) -> bool {
        const FILES: &[&str] = &[
            "cargo.lock", "package-lock.json", "yarn.lock", "pnpm-lock.yaml", "poetry.lock",
            "pipfile.lock", "composer.lock", "mix.lock", ".ds_store", "thumbs.db", "db.sqlite",
            "desktop.ini", ".eslintcache", ".stylelintcache", "npm-debug.log", "yarn-error.log",
        ];
        FILES.contains(&name)
    }

    /// Detects if a file is a binary artifact or too heavy based on its extension and size.
    fn is_heavy_artifact(entry: &DirEntry) -> bool {
        let path = entry.path();
        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("").to_lowercase();

        const FORBIDDEN: &[&str] = &[
            "exe", "dll", "so", "dylib", "wasm", "pdb", "zip", "tar", "gz", "7z", "png", "jpg",
            "jpeg", "gif", "svg", "ico", "mp4", "mov", "bin", "iso", "img", "msi", "dmg", "pkg",
            "sqlite", "pyc", "pyo", "pyd", "log", "bak", "swp", "tmp", "woff", "woff2", "ttf",
        ];

        if FORBIDDEN.contains(&ext.as_str()) {
            return true;
        }

        let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
        let limit = match ext.as_str() {
            "pdf" | "docx" | "xlsx" | "xls" => 1_000_000_000, // 1GB for office
            "xml" | "json" | "csv" | "sql" => 250_000_000,    // 250MB for data
            _ => 50_000_000,                                  // 50MB for source/text
        };

        size > limit
    }
}