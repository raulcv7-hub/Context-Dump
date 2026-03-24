use crate::core::config::ContextConfig;
use crate::ui::node::UiNode;
use ratatui::widgets::ListState;

pub struct App {
    pub nodes: Vec<UiNode>,
    pub root_indices: Vec<usize>,
    pub list_state: ListState,
    pub view_items: Vec<usize>,
    pub should_quit: bool,
    pub confirmed: bool,
    pub config: ContextConfig,
    pub default_filename: String,
}

impl App {
    /**
     * Marks the application to terminate the main loop.
     */
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    /**
     * Marks the current selection as final and prepares for exit.
     */
    pub fn confirm(&mut self) {
        self.confirmed = true;
        self.should_quit = true;
    }
}