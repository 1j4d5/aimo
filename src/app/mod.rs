pub mod state;
pub mod tabs;
pub mod welcome;
pub mod shortcuts;

pub use state::{EditorApp, BottomTab};
use eframe::{egui, App};
use crate::panels::top::TopBarCommand;

impl EditorApp {
    pub fn handle_command(&mut self, cmd: TopBarCommand) {
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
            TopBarCommand::EditTheme => {
                let theme_path = std::path::PathBuf::from("color.json");
                self.open_file_from_path(theme_path);
            }
            TopBarCommand::ReloadTheme => self.reload_theme(),
        }
    }
}

impl App for EditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // UI Style Setup
        let mut style = (*ctx.style()).clone();
        style.spacing.item_spacing = egui::vec2(0.0, 0.0);
        style.spacing.window_margin = egui::Margin::same(0); 
        ctx.set_style(style);

        // 1. Handle Commands
        let bar_cmd = crate::panels::top::show(ctx, &mut self.top_bar_state);
        let shortcut_cmd = shortcuts::handle(ctx);
        if let Some(cmd) = shortcut_cmd.or(bar_cmd) {
            self.handle_command(cmd);
        }

        // 2. Panels
        crate::panels::left_panel::show(ctx, self);
        crate::panels::bottom_panel::show(ctx, &mut self.bottom_tab, &self.theme);

        // 3. Central Area
        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(self.theme.col(self.theme.background)).inner_margin(0))
            .show(ctx, |ui| {
                if self.buffers.is_empty() {
                    if let Some(cmd) = welcome::show(self, ui) { self.handle_command(cmd); }
                } else {
                    tabs::show_tab_bar(self, ui);
                    ui.separator();
                    if let Some(buffer) = self.buffers.get_mut(self.active_buffer_idx) {
                        crate::code::editor::show(ui, &mut buffer.content, &self.theme, &mut buffer.is_dirty);
                    }
                }
            });

        // 4. Close Confirmation Modal
        if self.close_confirm.is_open {
            let idx = self.close_confirm.buffer_idx;
            let filename = self.buffers.get(idx).map(|b| b.name.clone()).unwrap_or_default();

            egui::Window::new("Unsaved Changes")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .show(ctx, |ui| {
                    ui.label(format!("'{}' has unsaved changes. Save before closing?", filename));
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        if ui.button("Save & Close").clicked() {
                            self.active_buffer_idx = idx;
                            self.save_current();
                            self.perform_close(idx);
                        }
                        if ui.button("Discard").clicked() {
                            self.perform_close(idx);
                        }
                        if ui.button("Cancel").clicked() {
                            self.close_confirm.is_open = false;
                        }
                    });
                });
        }
    }
}