use crate::ui::app::App;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List, ListItem};

/// Renders the hierarchical file explorer list with selection states and token indicators.
pub fn render_explorer(frame: &mut Frame, app: &mut App, area: Rect) {
    let max_tk = app.config.max_tokens_per_file;

    let items: Vec<ListItem> = app
        .view_items
        .iter()
        .map(|&idx| {
            let node = &app.nodes[idx];
            let indent = "  ".repeat(node.depth);
            let icon = if node.is_dir { "📂 " } else { "📄 " };
            let check = if node.selected { "[x] " } else { "[ ] " };

            let label = if node.is_dir {
                Span::styled(
                    format!("{}{}{}{}", indent, check, icon, node.name),
                    Style::default()
                        .fg(Color::Blue)
                        .add_modifier(Modifier::BOLD),
                )
            } else {
                let is_truncated = node.token_estimate > max_tk;

                let fg = if node.is_ignored || node.is_hidden {
                    Color::DarkGray
                } else if is_truncated {
                    Color::Magenta
                } else if node.token_estimate > 5000 {
                    Color::Red
                } else if node.token_estimate > 1000 {
                    Color::Yellow
                } else {
                    Color::Green
                };

                let truncate_label = if is_truncated { " [TRUNCATED]" } else { "" };

                Span::styled(
                    format!(
                        "{}{}{}{} ({} tk){}",
                        indent, check, icon, node.name, node.token_estimate, truncate_label
                    ),
                    Style::default().fg(fg),
                )
            };

            ListItem::new(Line::from(vec![label]))
        })
        .collect();

    frame.render_stateful_widget(
        List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Project Explorer "),
            )
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED)),
        area,
        &mut app.list_state,
    );
}