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
    pub search_query: String,
    pub search_mode: bool,
}

impl App {
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn confirm(&mut self) {
        self.confirmed = true;
        self.should_quit = true;
    }

    /// Toggles the search input mode.
    pub fn toggle_search(&mut self) {
        self.search_mode = !self.search_mode;
        if !self.search_mode {
            self.search_query.clear();
            self.update_view();
        }
    }
}
