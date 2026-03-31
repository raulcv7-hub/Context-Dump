use ignore::DirEntry;

/// Central module for detecting operational noise and sensitive security files.
pub struct NoiseDetector;

impl NoiseDetector {
    /// Determines if a file is considered sensitive or high-risk for exposure.
    pub fn is_sensitive(name: &str, ext: &str) -> bool {
        let exact_matches = [
            ".env",
            ".env.local",
            ".env.development",
            ".env.test",
            ".env.production",
            "id_rsa",
            "id_ed25519",
            "secrets.json",
            "auth.json",
            "credentials",
            ".npmrc",
            "kubeconfig",
            "passwd",
            "shadow",
            "htpasswd",
            "docker-compose.override.yml",
        ];

        let sensitive_ext = [
            "pem", "key", "p12", "pfx", "keystore", "pkpass", "ovpn", "kdbx", "jks",
        ];

        exact_matches.contains(&name) || sensitive_ext.contains(&ext) || name.starts_with(".env.")
    }

    /// Evaluates if a directory entry should be flagged as noise.
    pub fn is_noise(entry: &ignore::DirEntry) -> bool {
        let name = entry.file_name().to_string_lossy().to_lowercase();

        if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
            return Self::is_ignored_directory(&name);
        }

        Self::is_ignored_file(&name) || Self::is_heavy_artifact(entry)
    }

    /// Checks if a directory name matches common project exclusion patterns.
    fn is_ignored_directory(name: &str) -> bool {
        const DIRS: &[&str] = &[
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
            ".docker",
            ".k8s",
            ".expo",
            ".gradle",
            ".dart_tool",
            ".infrastructure",
            "platforms",
            "plugins",
            "Pods",
            ".docz",
            ".cache",
            ".pytest_cache",
            "site-packages",
        ];
        DIRS.contains(&name)
    }

    /// Checks if a file name matches common temporary or metadata file patterns.
    fn is_ignored_file(name: &str) -> bool {
        const FILES: &[&str] = &[
            "cargo.lock",
            "package-lock.json",
            "yarn.lock",
            "pnpm-lock.yaml",
            "poetry.lock",
            "pipfile.lock",
            "composer.lock",
            "mix.lock",
            ".ds_store",
            "thumbs.db",
            "db.sqlite",
            "desktop.ini",
            ".eslintcache",
            ".stylelintcache",
            "npm-debug.log",
            "yarn-error.log",
        ];
        FILES.contains(&name)
    }

    /// Detects if a file exceeds size limits or belongs to forbidden binary types.
    fn is_heavy_artifact(entry: &DirEntry) -> bool {
        if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
            return false;
        }

        let path = entry.path();
        let ext = path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_lowercase();

        const FORBIDDEN: &[&str] = &[
            "exe", "dll", "so", "dylib", "wasm", "pdb", "7z", "png", "jpg", "jpeg", "gif", "svg",
            "ico", "mp4", "mov", "bin", "iso", "img", "msi", "dmg", "pkg", "sqlite", "pyc", "pyo",
            "pyd", "log", "bak", "swp", "tmp", "woff", "woff2", "ttf",
        ];

        if FORBIDDEN.contains(&ext.as_str()) {
            return true;
        }

        let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
        let limit = match ext.as_str() {
            "pdf" | "docx" | "xlsx" | "xls" => 1_000_000_000,
            "xml" | "json" | "csv" | "sql" => 250_000_000,
            _ => 50_000_000,
        };

        size > limit
    }
}