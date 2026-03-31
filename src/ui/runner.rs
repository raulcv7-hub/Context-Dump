use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use std::collections::HashSet;
use std::io;
use std::path::{Path, PathBuf};

use crate::core::config::ContextConfig;
use crate::core::file::FileNode;
use crate::ui::app::App;
use crate::ui::view;

/// Initializes the interactive terminal user interface and controls the main rendering lifecycle.
pub fn run_tui(
    files: &[FileNode],
    root_path: &Path,
    initial_config: ContextConfig,
) -> Result<Option<(HashSet<PathBuf>, ContextConfig)>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(files, root_path, initial_config);
    let res = run_app_loop(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    res?;

    if app.confirmed {
        for node in &app.nodes {
            if !node.is_dir {
                app.config
                    .file_states
                    .insert(node.path.clone(), node.selected);
            }
        }
        Ok(Some((app.get_selected_paths(), app.config)))
    } else {
        Ok(None)
    }
}

/// Standard loop for handling TUI events and drawing frames.
fn run_app_loop(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> Result<()> {
    while !app.should_quit {
        terminal.draw(|f| view::render_app(f, app))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    handle_key_event(app, key.code);
                }
                Event::Mouse(mouse_event) => {
                    app.handle_mouse(mouse_event);
                }
                _ => {}
            }
        }
    }
    Ok(())
}

/// Maps keyboard input to application state actions.
fn handle_key_event(app: &mut App, code: KeyCode) {
    match code {
        KeyCode::Char('q') | KeyCode::Esc => app.quit(),
        KeyCode::Enter => app.confirm(),
        KeyCode::Char('f') => app.cycle_format(),
        KeyCode::Char('o') => app.cycle_output_destination(),
        KeyCode::Char('t') => app.toggle_tests(),
        KeyCode::Char('E') => app.expand_all(),
        KeyCode::Char('C') => app.collapse_all(),
        KeyCode::Char('e') => app.expand_node(),
        KeyCode::Char('c') => app.collapse_node(),
        KeyCode::Char('a') => app.select_all(),
        KeyCode::Char('d') => app.deselect_all(),
        KeyCode::Up => app.move_up(),
        KeyCode::Down => app.move_down(),
        KeyCode::Char(' ') => app.toggle_selection(),
        KeyCode::Right | KeyCode::Left => app.toggle_expand(),
        _ => {}
    }
}