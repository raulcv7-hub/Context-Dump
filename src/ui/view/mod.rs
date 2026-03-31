pub mod explorer;
pub mod panels;

use crate::ui::app::App;
use ratatui::prelude::*;

/// Orchestrates the high-level rendering of the TUI by dividing the frame into functional chunks.
pub fn render_app(frame: &mut Frame, app: &mut App) {
    let sensitive_count = app
        .nodes
        .iter()
        .filter(|n| n.is_sensitive && !n.is_dir)
        .count();

    let constraints = if sensitive_count > 0 {
        vec![
            Constraint::Min(0),
            Constraint::Length(3),
            Constraint::Length(6),
            Constraint::Length(4),
        ]
    } else {
        vec![
            Constraint::Min(0),
            Constraint::Length(6),
            Constraint::Length(4),
        ]
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(frame.area());

    explorer::render_explorer(frame, app, chunks[0]);

    if sensitive_count > 0 {
        panels::render_security_panel(frame, sensitive_count, chunks[1]);
        panels::render_summary_panel(frame, app, chunks[2]);
        panels::render_help_panel(frame, chunks[3]);
    } else {
        panels::render_summary_panel(frame, app, chunks[1]);
        panels::render_help_panel(frame, chunks[2]);
    }
}