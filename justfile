set shell := ["sh", "-c"]

p := `printf '\033[1;35m'`
c := `printf '\033[0;36m'`
r := `printf '\033[1;31m'`
reset := `printf '\033[0m'`

clippy_base := "cargo clippy --all-targets --message-format=short -- -D warnings "

default:
    @just --list

# Automatically fixes formatting and safe clippy suggestions
fix:
    @echo "{{p}}🛠️  Applying automatic fixes...{{reset}}"
    cargo fmt --all
    cargo clippy --fix --allow-dirty --allow-staged --all-targets -- -D warnings
    @echo "{{p}}✅ Safe fixes applied.{{reset}}"

# Runs formatting, then all audits, and finally the test suite
ci: fmt audit-all test
    @echo "{{p}}✨ Continuous Integration pipeline completed successfully.{{reset}}"

# Formats the code using cargo fmt
fmt:
    @echo "{{c}}🎨 Formatting code...{{reset}}"
    cargo fmt --all

# Runs the complete test suite
test: fmt
    @echo "{{c}}🧪 Running test suite...{{reset}}"
    cargo test

# Runs tests marked as ignored (heavy tests)
test-ignored: fmt
    @echo "{{c}}🧪 Running ignored tests...{{reset}}"
    cargo test -- --ignored

# Executes all quality and design audits
audit-all: fmt safety performance api-hygiene srp kiss solid yagni dry

# Checks for unsafe code and unwrap usage
safety:
    @echo "{{r}}🛡️  [Safety] Verifying safety invariants...{{reset}}"
    @{{clippy_base}} -W clippy::unwrap_used -W clippy::expect_used
    @if command -v cargo-audit > /dev/null; then cargo audit; fi

# Detects inefficient patterns and unnecessary clones
performance:
    @echo "{{c}}⚡ [Performance] Searching for inefficiencies...{{reset}}"
    @{{clippy_base}} -W clippy::clone_on_ref_ptr -W clippy::inefficient_to_string

# Verifies public API contracts and error documentation
api-hygiene:
    @echo "{{c}}📖 [API Hygiene] Verifying public documentation...{{reset}}"
    @{{clippy_base}} -W clippy::missing_errors_doc

# Verifies function and class cohesion (Single Responsibility)
srp:
    @echo "{{c}}🔎 [SRP] Verifying function cohesion...{{reset}}"
    @{{clippy_base}} -W clippy::too_many_lines -W clippy::too_many_arguments

# Checks cyclomatic and cognitive complexity
kiss:
    @echo "{{c}}🔎 [KISS] Verifying code complexity...{{reset}}"
    @{{clippy_base}} -W clippy::cognitive_complexity -W clippy::excessive_nesting

# Verifies coupling and type design
solid:
    @echo "{{c}}🔎 [SOLID] Verifying design and coupling...{{reset}}"
    @{{clippy_base}} -W clippy::type_complexity

# Detects dead code and unused dependencies
yagni:
    @echo "{{c}}🔎 [YAGNI] Removing dead code...{{reset}}"
    @if command -v cargo-machete > /dev/null; then cargo machete; fi

# Searches for logical duplication
dry:
    @echo "{{c}}🔎 [DRY] Searching for logical duplication...{{reset}}"
    @{{clippy_base}} -W clippy::branches_sharing_code -W clippy::ifs_same_cond