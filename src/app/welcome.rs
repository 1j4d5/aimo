use eframe::egui;
use crate::app::state::EditorApp;
use crate::panels::top::TopBarCommand;

pub fn show(app: &EditorApp, ui: &mut egui::Ui) -> Option<TopBarCommand> {
    let mut cmd = None;

    ui.vertical_centered(|ui| {
        ui.add_space(ui.available_height() / 3.0);
        
        ui.heading(
            egui::RichText::new("🚀 Ijaduim NJIN")
                .size(40.0)
                .strong()
                .color(app.theme.col(app.theme.keywords))
        );
        
        ui.label("The lightweight Rust IDE");
        ui.add_space(20.0);

        ui.group(|ui| {
            ui.set_width(320.0);
            ui.vertical(|ui| {
                ui.add_space(10.0);
                if ui.button("📄 New File                Ctrl+N").clicked() {
                    cmd = Some(TopBarCommand::NewFile);
                }
                ui.add_space(5.0);
                if ui.button("📂 Open File               Ctrl+O").clicked() {
                    cmd = Some(TopBarCommand::OpenFile);
                }
                // ADD THIS:
                ui.add_space(5.0);
                if ui.button("📁 Open Folder       Ctrl+Shift+O").clicked() {
                    cmd = Some(TopBarCommand::OpenFolder);
                }
                ui.add_space(10.0);
            });
        });
    });

    cmd
}