use eframe::egui;
// Try this path if crate::app::BottomTab fails
pub use crate::app::BottomTab; 
use crate::code::theme::EditorTheme;

pub fn show(ctx: &egui::Context, bottom_tab: &mut BottomTab, theme: &EditorTheme) {
    egui::TopBottomPanel::bottom("bottom_panel")
        .default_height(160.0)
        .resizable(true)
        .frame(egui::Frame::default().fill(theme.col(theme.background)))
        .show(ctx, |ui| {
            // Apply theme text color to the whole panel
            ui.visuals_mut().override_text_color = Some(theme.col(theme.text));

            ui.horizontal(|ui| {
                ui.selectable_value(bottom_tab, BottomTab::Terminal, "Terminal");
                ui.selectable_value(bottom_tab, BottomTab::Problems, "Problems");
            });
            
            ui.separator();

            match bottom_tab {
                BottomTab::Terminal => {
                    ui.monospace("> cargo build --release");
                }
                BottomTab::Problems => {
                    ui.label("No problems detected");
                }
            }
        });
}