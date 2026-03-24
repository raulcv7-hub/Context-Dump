use crate::ui::app::App;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

/**
 * Renders the full application frame.
 */
pub fn render_app(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),    // Tree Explorer
            Constraint::Length(5), // Stats Summary
            Constraint::Length(3), // Controls Help
        ])
        .split(frame.area());

    let items: Vec<ListItem> = app
        .view_items
        .iter()
        .map(|&idx| {
            let node = &app.nodes[idx];
            let indent = "  ".repeat(node.depth);
            let icon = if node.is_dir { "📂 " } else { "📄 " };
            let check = if node.selected { "[x] " } else { "[ ] " };
            let mut style = Style::default();
            if node.is_ignored || node.is_hidden {
                style = style.fg(Color::DarkGray);
            }
            ListItem::new(format!("{}{}{}{}", indent, check, icon, node.name)).style(style)
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
        chunks[0],
        &mut app.list_state,
    );

    let total_tokens = app.get_total_selected_tokens();
    let selected_count = app.get_selected_count();
    let dist = app.get_language_distribution();
    let dist_str = dist
        .iter()
        .take(5)
        .map(|(l, p)| format!("{}: {:.0}%", l.to_uppercase(), p))
        .collect::<Vec<_>>()
        .join(" | ");

    let output_dest = match &app.config.output_path {
        Some(path) => path.to_string_lossy().to_string(),
        None => "TERMINAL (STDOUT)".to_string(),
    };

    let stats_lines = vec![
        Line::from(vec![
            Span::raw(format!(
                "Selected: {} files | Tokens: {} ",
                selected_count, total_tokens
            ))
        ]),
        Line::from(vec![
            Span::styled("Output: ", Style::default().fg(Color::Green)),
            Span::styled(output_dest, Style::default().fg(Color::White)),
            Span::raw(" | "),
            Span::styled("Format: ", Style::default().fg(Color::Cyan)),
            Span::raw(format!("{:?} ", app.config.output_format)),
            Span::styled("Clipboard: ", Style::default().fg(Color::Cyan)),
            Span::raw(if app.config.to_clipboard { "ON" } else { "OFF" }),
        ]),
        Line::from(vec![
            Span::styled("Top Languages: ", Style::default().fg(Color::Yellow)),
            Span::raw(dist_str),
        ]),
    ];

    frame.render_widget(
        Paragraph::new(stats_lines).block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Context Summary "),
        ),
        chunks[1],
    );

    let help_text = " [Esc/q] Quit | [Space] Select | [Enter] Process | [Arrows] Navigate | [c] Clip | [o] Output | [f] Format ";
    frame.render_widget(
        Paragraph::new(help_text)
            .block(Block::default().borders(Borders::ALL).title(" Controls "))
            .style(Style::default().fg(Color::Cyan)),
        chunks[2],
    );
}