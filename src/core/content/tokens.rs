use tiktoken_rs::{cl100k_base, CoreBPE};
use std::sync::OnceLock;

/// Static storage for the BPE encoder to avoid expensive re-initialization.
static ENCODER: OnceLock<CoreBPE> = OnceLock::new();

/// Utility for precise token counting using BPE encoding.
pub struct TokenCounter;

impl TokenCounter {
    /// Counts tokens using the cl100k_base encoding.
    /// Uses OnceLock to ensure the heavy encoder is only loaded once during the process lifetime.
    pub fn count(text: &str) -> usize {
        let bpe = ENCODER.get_or_init(|| {
            cl100k_base().expect("Failed to initialize cl100k_base encoder")
        });
        bpe.encode_with_special_tokens(text).len()
    }

    /// Estimates token count for a given file size in bytes as a fast fallback.
    pub fn estimate_from_size(bytes: u64) -> usize {
        (bytes as usize) / 4
    }
}