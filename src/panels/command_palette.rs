use eframe::egui;

pub fn show(ctx: &egui::Context, show_palette: &mut bool) {
    if *show_palette {
        egui::Window::new("Command Palette")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_TOP, [0.0, 40.0])
            .show(ctx, |ui| {
                ui.label("Type a command...");
                ui.separator();
                if ui.button("Open File").clicked() {
                    *show_palette = false;
                }
                if ui.button("Save File").clicked() {
                    *show_palette = false;
                }
            });
    }
}
