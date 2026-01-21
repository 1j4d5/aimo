use crate::panels::top::TopBarState; // Removed unused imports to fix warnings
use crate::code::theme::EditorTheme;
use std::path::PathBuf;

pub struct FileBuffer {
    pub name: String,
    pub path: Option<String>,
    pub content: String,
}

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
}

impl EditorApp {
    // THIS FIXES THE E0599 ERROR
    pub fn reload_theme(&mut self) {
        self.theme = EditorTheme::load();
    }

    pub fn new_file(&mut self) {
        self.buffers.push(FileBuffer {
            name: format!("untitled_{}.rs", self.buffers.len() + 1),
            path: None,
            content: String::new(),
        });
        self.active_buffer_idx = self.buffers.len() - 1;
    }

    pub fn open_folder(&mut self, path: PathBuf) {
        self.project_path = Some(path);
    }

    pub fn open_file_from_path(&mut self, path: PathBuf) {
        let path_str = path.to_string_lossy().to_string();
        if let Some(idx) = self.buffers.iter().position(|b| b.path.as_ref() == Some(&path_str)) {
            self.active_buffer_idx = idx;
            return;
        }
        if let Ok(content) = std::fs::read_to_string(&path) {
            let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
            self.buffers.push(FileBuffer { name, path: Some(path_str), content });
            self.active_buffer_idx = self.buffers.len() - 1;
        }
    }

    pub fn save_current(&mut self) {
        if let Some(buffer) = self.buffers.get_mut(self.active_buffer_idx) {
            if let Some(path) = &buffer.path {
                let _ = std::fs::write(path, &buffer.content);
                if buffer.name == "color.json" { self.reload_theme(); }
            } else if let Some(path) = rfd::FileDialog::new().save_file() {
                if std::fs::write(&path, &buffer.content).is_ok() {
                    buffer.path = Some(path.to_string_lossy().to_string());
                    buffer.name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                }
            }
        }
    }

    pub fn close_buffer(&mut self, idx: usize) {
        if idx < self.buffers.len() {
            self.buffers.remove(idx);
            if !self.buffers.is_empty() {
                self.active_buffer_idx = self.active_buffer_idx.min(self.buffers.len() - 1);
            }
        }
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
        }
    }
}