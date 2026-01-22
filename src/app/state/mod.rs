pub mod buffer;
pub mod confirm;

pub use buffer::FileBuffer;
pub use confirm::CloseConfirmation;

use std::path::PathBuf;
use crate::panels::top::TopBarState;
use crate::code::theme::EditorTheme;

#[derive(PartialEq, Default, Clone, Copy)]
pub enum BottomTab {
    #[default] Terminal,
    Problems,
}

pub struct EditorApp {
    pub buffers: Vec<FileBuffer>,
    pub active_buffer_idx: usize,
    pub theme: EditorTheme,
    pub bottom_tab: BottomTab,
    pub top_bar_state: TopBarState,
    pub project_path: Option<PathBuf>,
    pub close_confirm: CloseConfirmation,
}

impl EditorApp {
    pub fn reload_theme(&mut self) {
        self.theme = EditorTheme::load();
    }

    pub fn new_file(&mut self) {
        self.buffers.push(FileBuffer::new_untitled(self.buffers.len() + 1));
        self.active_buffer_idx = self.buffers.len() - 1;
    }

    pub fn open_file_from_path(&mut self, path: PathBuf) {
        let path_str = path.to_string_lossy().to_string();
        // Don't reopen if already open
        if let Some(idx) = self.buffers.iter().position(|b| b.path.as_ref() == Some(&path_str)) {
            self.active_buffer_idx = idx;
            return;
        }
        if let Some(new_buffer) = FileBuffer::from_path(path) {
            self.buffers.push(new_buffer);
            self.active_buffer_idx = self.buffers.len() - 1;
        }
    }
    pub fn open_folder(&mut self, path: PathBuf) {
        self.project_path = Some(path);
    }
    pub fn save_current(&mut self) {
        let mut reload_needed = false;
        if let Some(buffer) = self.buffers.get_mut(self.active_buffer_idx) {
            if buffer.save() && buffer.name == "color.json" {
                reload_needed = true;
            }
        }
        if reload_needed { self.reload_theme(); }
    }

    pub fn close_buffer(&mut self, idx: usize) {
        if let Some(buffer) = self.buffers.get(idx) {
            if buffer.is_dirty {
                self.close_confirm.ask(idx);
            } else {
                self.perform_close(idx);
            }
        }
    }

    pub fn perform_close(&mut self, idx: usize) {
        if idx < self.buffers.len() {
            self.buffers.remove(idx);
            if !self.buffers.is_empty() {
                self.active_buffer_idx = self.active_buffer_idx.min(self.buffers.len() - 1);
            }
        }
        self.close_confirm.close();
    }
}

impl Default for EditorApp {
    fn default() -> Self {
        Self {
            buffers: Vec::new(),
            active_buffer_idx: 0,
            theme: EditorTheme::load(),
            bottom_tab: BottomTab::default(),
            top_bar_state: TopBarState::default(),
            project_path: None,
            close_confirm: CloseConfirmation::default(),
        }
    }
}