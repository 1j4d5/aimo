use eframe::egui;
use crate::app::state::EditorApp;
use crate::panels::top::TopBarCommand;

pub fn show(app: &EditorApp, ui: &mut egui::Ui) -> Option<TopBarCommand> {
    let mut cmd = None;

    ui.vertical_centered(|ui| {
        // Position the content about 1/3 down the screen
        ui.add_space(ui.available_height() / 3.0);
        
        // Main Heading using theme colors
        ui.heading(
            egui::RichText::new("🚀 Ijaduim NJIN")
                .size(40.0)
                .strong()
                .color(app.theme.col(app.theme.keywords))
        );
        
        ui.label("The lightweight Rust IDE");
        ui.add_space(20.0);

        // Define a uniform button size for a cleaner look
        let button_size = egui::vec2(300.0, 32.0);

        // Group the action buttons
        ui.group(|ui| {
            ui.set_width(320.0);
            ui.vertical(|ui| {
                ui.add_space(10.0);

                // New File Button
                if ui.add(egui::Button::new("📄 New File                Ctrl+N")
                    .min_size(button_size))
                    .clicked() 
                {
                    cmd = Some(TopBarCommand::NewFile);
                }

                ui.add_space(8.0);

                // Open File Button
                if ui.add(egui::Button::new("📂 Open File               Ctrl+O")
                    .min_size(button_size))
                    .clicked() 
                {
                    cmd = Some(TopBarCommand::OpenFile);
                }

                ui.add_space(8.0);

                // Open Folder Button
                if ui.add(egui::Button::new("📁 Open Folder       Ctrl+Shift+O")
                    .min_size(button_size))
                    .clicked() 
                {
                    cmd = Some(TopBarCommand::OpenFolder);
                }

                ui.add_space(10.0);
            });
        });
    });

    // Return the command so mod.rs can call self.handle_command(cmd)
    cmd
}