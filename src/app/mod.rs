pub mod state;
pub mod tabs;
pub mod welcome;
pub mod shortcuts;

pub use state::{EditorApp, BottomTab};
use eframe::{egui, App};
use crate::panels::top::TopBarCommand;

impl App for EditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut style = (*ctx.style()).clone();
        style.spacing.item_spacing = egui::vec2(0.0, 0.0);
        style.spacing.window_margin = egui::Margin::same(0); // Fixed i8
        ctx.set_style(style);

        let bar_cmd = crate::panels::top::show(ctx, &mut self.top_bar_state);
        let shortcut_cmd = shortcuts::handle(ctx);
        let active_cmd = shortcut_cmd.or(bar_cmd);

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
                TopBarCommand::EditTheme => {
                    let theme_path = std::path::PathBuf::from("color.json");
                    self.open_file_from_path(theme_path);
                }
                TopBarCommand::ReloadTheme => self.reload_theme(),
            }
        }

        crate::panels::left_panel::show(ctx, self);
        crate::panels::bottom_panel::show(ctx, &mut self.bottom_tab, &self.theme);

        egui::CentralPanel::default()
            .frame(egui::Frame::default()
                .fill(self.theme.col(self.theme.background))
                .inner_margin(0) // Fixed i8
            )
            .show(ctx, |ui| {
                if self.buffers.is_empty() {
                    welcome::show(self, ui);
                } else {
                    tabs::show_tab_bar(self, ui);
                    ui.separator();
                    if let Some(buffer) = self.buffers.get_mut(self.active_buffer_idx) {
                        crate::code::editor::show(ui, &mut buffer.content, &self.theme);
                    }
                }
            });
    }
}