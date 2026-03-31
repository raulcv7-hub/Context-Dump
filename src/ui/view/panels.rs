use crate::ui::app::App;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

/// Renders a critical alert panel when locked sensitive assets are detected in the filesystem.
pub fn render_security_panel(frame: &mut Frame, count: usize, area: Rect) {
    let msg = format!(" ⚠️ SECURITY ALERT: {} sensitive files detected and LOCKED. They are visible but cannot be selected.", count);
    frame.render_widget(
        Paragraph::new(msg).block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Red)),
        ),
        area,
    );
}

/// Renders the operational metrics, destinations, and token status context box.
pub fn render_summary_panel(frame: &mut Frame, app: &mut App, area: Rect) {
    let dist = app.get_language_distribution();
    let dist_str = dist
        .iter()
        .take(5)
        .map(|(lang, pct)| format!("{}: {:.0}%", lang.to_uppercase(), pct))
        .collect::<Vec<String>>()
        .join(" | ");

    let (dest_label, dest_style) = if app.config.to_clipboard {
        (
            "SYSTEM CLIPBOARD",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
    } else if let Some(path) = &app.config.output_path {
        (
            path.to_str().unwrap_or("FILE"),
            Style::default().fg(Color::White),
        )
    } else {
        ("TERMINAL (STDOUT)", Style::default().fg(Color::DarkGray))
    };

    let stats_lines = vec![
        Line::from(vec![Span::raw(format!(
            "Selected: {} files | Tokens: {}",
            app.get_selected_count(),
            app.get_total_selected_tokens()
        ))]),
        Line::from(vec![
            Span::styled("Output Destination: ", Style::default().fg(Color::Green)),
            Span::styled(dest_label, dest_style),
            Span::raw(" | "),
            Span::styled("Format: ", Style::default().fg(Color::Cyan)),
            Span::raw(format!("{:?} ", app.config.output_format)),
        ]),
        Line::from(vec![
            Span::styled("Top Languages: ", Style::default().fg(Color::LightBlue)),
            Span::raw(dist_str),
        ]),
        Line::from(vec![
            Span::styled(
                "Legend: ",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" < 1k tk ", Style::default().fg(Color::Green)),
            Span::raw("|"),
            Span::styled(" < 5k tk ", Style::default().fg(Color::Yellow)),
            Span::raw("|"),
            Span::styled(" > 5k tk ", Style::default().fg(Color::Red)),
            Span::raw("|"),
            Span::styled(
                format!(" > {} tk [TRUNCATED]", app.config.max_tokens_per_file),
                Style::default().fg(Color::Magenta),
            ),
        ]),
    ];

    frame.render_widget(
        Paragraph::new(stats_lines).block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Context Summary "),
        ),
        area,
    );
}

/// Renders the navigational aid map at the base of the interface.
pub fn render_help_panel(frame: &mut Frame, area: Rect) {
    let help_lines = vec![
        Line::from(
            " [Esc/q] Quit | [Space] Toggle File | [Enter] Go | [a/d] All/None | [t] Toggle Tests",
        ),
        Line::from(
            " [E/e] Expand All/Node | [C/c] Collapse All/Node | [o/f] Toggle Outputs/Formats ",
        ),
    ];
    frame.render_widget(
        Paragraph::new(help_lines)
            .block(Block::default().borders(Borders::ALL).title(" Controls "))
            .style(Style::default().fg(Color::Cyan)),
        area,
    );
}