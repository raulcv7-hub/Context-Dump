use crate::ui::app::App;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

pub fn render_app(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Search bar
            Constraint::Min(0),    // Tree
            Constraint::Length(6), // Stats (Incrementado de 5 a 6 para nueva línea)
            Constraint::Length(3), // Help
        ])
        .split(frame.area());

    let search_style = if app.search_mode {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::DarkGray)
    };
    let search_text = if app.search_query.is_empty() && !app.search_mode {
        " Press [/] to search...".to_string()
    } else {
        format!(" Query: {}", app.search_query)
    };

    frame.render_widget(
        Paragraph::new(search_text).block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Quick Search ")
                .border_style(search_style),
        ),
        chunks[0],
    );

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
        chunks[1],
        &mut app.list_state,
    );

    let total_tokens = app.get_total_selected_tokens();
    let selected_count = app.get_selected_count();
    let ignored_count = app.get_smart_ignored_count();
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
            )),
            Span::styled(
                format!("(Ignored {} by Smart Heuristics)", ignored_count),
                Style::default().fg(Color::Magenta),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "Output To: ",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(output_dest, Style::default().fg(Color::White)),
            Span::raw(" | "),
            Span::styled("Format: ", Style::default().fg(Color::Cyan)),
            Span::raw(format!("{:?} ", app.config.output_format)),
        ]),
        Line::from(vec![
            Span::styled("Minify: ", Style::default().fg(Color::Cyan)),
            Span::raw(if app.config.minify { "ON " } else { "OFF " }),
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
                .title(" Pre-visualization Summary "),
        ),
        chunks[2],
    );

    let help_text = " [/] Search | [Esc] Clear | [Space] Toggle | [Enter] Confirm | [c] Clip | [m] Minify | [o] File/Stdout | [f] Format ";
    frame.render_widget(
        Paragraph::new(help_text)
            .block(Block::default().borders(Borders::ALL).title(" Controls "))
            .style(Style::default().fg(Color::Cyan)),
        chunks[3],
    );
}
