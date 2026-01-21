use std::fs;
use rfd::FileDialog;

pub struct OpenedFile {
    pub path: String,
    pub content: String,
}

pub fn open_file_dialog() -> Option<OpenedFile> {
    let file = FileDialog::new()
        .add_filter("Rust Files", &["rs"])
        .add_filter("JSON Files", &["json"])
        .add_filter("All Files", &["*"])
        .pick_file();

    if let Some(path) = file {
        if let Ok(content) = fs::read_to_string(&path) {
            return Some(OpenedFile {
                path: path.display().to_string(),
                content,
            });
        }
    }
    None
}