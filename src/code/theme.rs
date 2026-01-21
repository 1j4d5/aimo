use eframe::egui::Color32;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EditorTheme {
    pub keywords: [u8; 3],
    pub types: [u8; 3],
    pub strings: [u8; 3],
    pub comments: [u8; 3],
    pub background: [u8; 3],
    pub text: [u8; 3],
}

impl Default for EditorTheme {
    fn default() -> Self {
        Self {
            keywords: [198, 120, 221], // Purple
            types: [209, 154, 102],    // Orange
            strings: [152, 195, 121],  // Green
            comments: [92, 99, 112],   // Gray
            background: [40, 44, 104],  // Dark Gray
            text: [171, 178, 191],     // Light Gray
        }
    }
}

impl EditorTheme {
    pub fn load() -> Self {
        let path = "color.json";
        if let Ok(content) = fs::read_to_string(path) {
            if let Ok(theme) = serde_json::from_str(&content) {
                println!("Theme loaded from {}", path);
                return theme;
            }
        }
        
        // If file doesn't exist, create a default one
        let default_theme = Self::default();
        let _ = fs::write(path, serde_json::to_string_pretty(&default_theme).unwrap());
        default_theme
    }

    // Helper to convert [u8;3] to Color32
    pub fn col(&self, field: [u8; 3]) -> Color32 {
        Color32::from_rgb(field[0], field[1], field[2])
    }
}