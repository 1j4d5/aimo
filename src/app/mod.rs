pub mod state;
pub mod tabs;
pub mod welcome;
pub mod shortcuts;

pub use state::{EditorApp, FileBuffer, BottomTab};

use eframe::{egui, App};
use crate::panels::top::TopBarCommand;

impl App for EditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 1. INPUT HANDLING
        // Get commands from the custom titlebar/menu system
        let bar_cmd = crate::panels::top::show(ctx, &mut self.top_bar_state);
        
        // Get commands from keyboard shortcuts (Ctrl+S, etc.)
        let shortcut_cmd = shortcuts::handle(ctx);
        
        // Combine commands (Shortcuts usually take priority)
        let active_cmd = shortcut_cmd.or(bar_cmd);

        // 2. COMMAND PROCESSING
        if let Some(cmd) = active_cmd {
            match cmd {
                TopBarCommand::NewFile => self.new_file(),
                TopBarCommand::OpenFolder => {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        self.open_folder(path);
                    }
                }
                TopBarCommand::OpenFile => {
                    if let Some(opened) = crate::code::open::open_file_dialog() {
                        self.open_file_from_path(std::path::PathBuf::from(opened.path));
                    }
                }
                TopBarCommand::SaveFile => self.save_current(),
                TopBarCommand::CloseTab => self.close_buffer(self.active_buffer_idx),
                
                // Theme Management
                TopBarCommand::EditTheme => {
                    let theme_path = std::path::PathBuf::from("color.json");
                    self.open_file_from_path(theme_path);
                }
                TopBarCommand::ReloadTheme => {
                    self.reload_theme();
                }
            }
        }

        // 3. SIDE & BOTTOM PANELS
        // Left Panel handles the recursive file explorer
        crate::panels::left_panel::show(ctx, self);
        
        // Bottom Panel handles terminal/problems output
        crate::panels::bottom_panel::show(ctx, &mut self.bottom_tab, &self.theme);

        // 4. CENTRAL CONTENT AREA
        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(self.theme.col(self.theme.background)))
            .show(ctx, |ui| {
                if self.buffers.is_empty() {
                    // Show Welcome Screen if no files are open
                    if let Some(screen_cmd) = welcome::show(self, ui) {
                        match screen_cmd {
                            TopBarCommand::NewFile => self.new_file(),
                            TopBarCommand::OpenFolder => {
                                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                                    self.open_folder(path);
                                }
                            }
                            TopBarCommand::OpenFile => {
                                if let Some(opened) = crate::code::open::open_file_dialog() {
                                    self.open_file_from_path(std::path::PathBuf::from(opened.path));
                                }
                            }
                            // Ignore other commands (like Save/Reload) on the welcome screen
                            _ => {}
                        }
                    }
                } else {
                    // Show Tabs and Code Editor
                    tabs::show_tab_bar(self, ui);
                    ui.separator();
                    
                    if let Some(buffer) = self.buffers.get_mut(self.active_buffer_idx) {
                        crate::code::editor::show(ui, &mut buffer.content, &self.theme);
                    }
                }
            });
    }
}