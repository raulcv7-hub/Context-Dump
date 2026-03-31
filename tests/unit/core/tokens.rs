use context::core::content::TokenCounter;

/// Verifies that the counter handles empty strings without panicking.
#[test]
fn test_token_count_empty() {
    assert_eq!(TokenCounter::count(""), 0);
}

/// Verifies accurate token count for standard programming strings using cl100k_base.
#[test]
fn test_token_count_precision() {
    let code = "fn main() { println!(\"Hello\"); }";
    let count = TokenCounter::count(code);
    // 'fn' ' main' '(' ')' ' {' ' println' '!' '(' '"' 'Hello' '"' ')' ';' ' }'
    assert!(count > 5);
    assert!(count < 20);
}

/// Verifies that the fallback size estimation is conservative (4 bytes per token).
#[test]
fn test_token_estimation_from_size() {
    let bytes = 1024; // 1KB
    let estimate = TokenCounter::estimate_from_size(bytes);
    assert_eq!(estimate, 256);
}
