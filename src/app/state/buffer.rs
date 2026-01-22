use std::path::PathBuf;
use crate::code::theme::EditorTheme;

pub struct FileBuffer {
    pub name: String,
    pub path: Option<String>,
    pub content: String,
    pub is_dirty: bool,
}

impl FileBuffer {
    pub fn new_untitled(id: usize) -> Self {
        Self {
            name: format!("untitled_{}.rs", id),
            path: None,
            content: String::new(),
            is_dirty: true,
        }
    }

    pub fn from_path(path: PathBuf) -> Option<Self> {
        if let Ok(content) = std::fs::read_to_string(&path) {
            let name = path.file_name()?.to_string_lossy().to_string();
            let path_str = path.to_string_lossy().to_string();
            Some(Self {
                name,
                path: Some(path_str),
                content,
                is_dirty: false,
            })
        } else {
            None
        }
    }

    pub fn save(&mut self) -> bool {
        let mut success = false;
        if let Some(path) = &self.path {
            if std::fs::write(path, &self.content).is_ok() {
                success = true;
            }
        } else if let Some(path) = rfd::FileDialog::new().save_file() {
            if std::fs::write(&path, &self.content).is_ok() {
                self.path = Some(path.to_string_lossy().to_string());
                self.name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                success = true;
            }
        }

        if success {
            self.is_dirty = false;
        }
        success
    }
}