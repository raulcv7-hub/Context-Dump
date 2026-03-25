use crate::engine::summary;
use context::adapters::fs_reader::FsReader;
use context::core::config::ContextConfig;
use context::core::file::FileNode;
use context::ports::reader::FileReader;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use tracing::{info, warn};

/// Executes the concurrent ingestion and processing of file nodes.
/// Uses Rayon for data parallelism and a shared progress bar for feedback.
pub fn run(nodes: Vec<FileNode>, config: ContextConfig) -> anyhow::Result<()> {
    if nodes.is_empty() {
        warn!("No files selected for processing.");
        return Ok(());
    }

    info!("Starting parallel ingestion of {} files...", nodes.len());
    let pb = create_progress_bar(nodes.len());
    let reader = FsReader::new();
    let contexts: Vec<_> = nodes.into_par_iter().map(|node| {
        let ctx = reader.read_file(&node, &config);
        pb.inc(1);
        ctx
    }).collect();

    pb.finish_and_clear();
    summary::print(&contexts, &config)
}

/// Creates a progress bar configured for thread-safe updates.
fn create_progress_bar(len: usize) -> ProgressBar {
    let pb = ProgressBar::new(len as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("#>-"),
    );
    pb
}