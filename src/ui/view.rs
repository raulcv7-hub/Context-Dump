use crate::ui::app::App;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

/// Renders the main view of the TUI application.
pub fn render_app(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),    // Tree
            Constraint::Length(5), // Summary
            Constraint::Length(3), // Help
        ])
        .split(frame.area());

    let items: Vec<ListItem> = app.view_items.iter().map(|&idx| {
        let node = &app.nodes[idx];
        let indent = "  ".repeat(node.depth);
        let icon = if node.is_dir { "📂 " } else { "📄 " };
        let check = if node.selected { "[x] " } else { "[ ] " };
        let style = if node.is_ignored || node.is_hidden {
            Style::default().fg(Color::DarkGray)
        } else {
            Style::default()
        };
        ListItem::new(format!("{}{}{}{}", indent, check, icon, node.name)).style(style)
    }).collect();

    frame.render_stateful_widget(
        List::new(items).block(Block::default().borders(Borders::ALL).title(" Project Explorer "))
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED)),
        chunks[0], &mut app.list_state,
    );

    render_summary_panel(frame, app, chunks[1]);
    render_help_panel(frame, chunks[2]);
}

/// Renders the summary statistics and the current output destination.
fn render_summary_panel(frame: &mut Frame, app: &mut App, area: Rect) {
    let dist = app.get_language_distribution();
    let dist_str = dist.iter().take(5)
        .map(|(lang, pct): &(String, f64)| format!("{}: {:.0}%", lang.to_uppercase(), pct))
        .collect::<Vec<String>>().join(" | ");

    let (dest_label, dest_style) = if app.config.to_clipboard {
        ("SYSTEM CLIPBOARD", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
    } else if let Some(path) = &app.config.output_path {
        (path.to_str().unwrap_or("FILE"), Style::default().fg(Color::White))
    } else {
        ("TERMINAL (STDOUT)", Style::default().fg(Color::DarkGray))
    };

    let stats_lines = vec![
        Line::from(vec![
            Span::raw(format!("Selected: {} files | Tokens: {} (Precision cl100k_base)", 
                app.get_selected_count(), app.get_total_selected_tokens())),
        ]),
        Line::from(vec![
            Span::styled("Output Destination: ", Style::default().fg(Color::Green)),
            Span::styled(dest_label, dest_style),
            Span::raw(" | "),
            Span::styled("Format: ", Style::default().fg(Color::Cyan)),
            Span::raw(format!("{:?} ", app.config.output_format)),
        ]),
        Line::from(vec![
            Span::styled("Top Languages: ", Style::default().fg(Color::Yellow)),
            Span::raw(dist_str),
        ]),
    ];

    frame.render_widget(
        Paragraph::new(stats_lines).block(Block::default().borders(Borders::ALL).title(" Context Summary ")),
        area,
    );
}

/// Renders the keyboard controls bar.
fn render_help_panel(frame: &mut Frame, area: Rect) {
    let help_text = " [Esc/q] Quit | [Space] Select | [Enter] Process | [Arrows] Navigate | [o] Output Mode | [f] Format ";
    frame.render_widget(
        Paragraph::new(help_text).block(Block::default().borders(Borders::ALL).title(" Controls "))
            .style(Style::default().fg(Color::Cyan)),
        area,
    );
}