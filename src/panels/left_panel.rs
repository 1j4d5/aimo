use eframe::egui;
use crate::app::state::EditorApp;
use std::fs;
use std::path::Path;

pub fn show(ctx: &egui::Context, app: &mut EditorApp) {
    egui::SidePanel::left("left_panel")
        .default_width(220.0)
        .resizable(true)
        .show(ctx, |ui| {
            ui.add_space(10.0);
            ui.heading("📁 Explorer");
            ui.separator();
            ui.add_space(5.0);

            if let Some(root_path) = app.project_path.clone() {
                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        render_tree(ui, &root_path, app);
                    });
            } else {
                // --- FIXED NO PROJECT STATE ---
                ui.centered_and_justified(|ui| {
                    ui.vertical_centered(|ui| {
                        ui.label(egui::RichText::new("No project folder open").weak());
                        ui.add_space(10.0);
                        
                        // Fixed size button: 120px wide, 30px high
                        if ui.add_sized([120.0, 30.0], egui::Button::new("Open Folder")).clicked() {
                            if let Some(path) = rfd::FileDialog::new().pick_folder() {
                                app.open_folder(path);
                            }
                        }
                    });
                });
            }
        });
}

fn render_tree(ui: &mut egui::Ui, path: &Path, app: &mut EditorApp) {
    if let Ok(entries) = fs::read_dir(path) {
        let mut entries: Vec<_> = entries.flatten().collect();
        // Sort: Folders first
        entries.sort_by(|a, b| {
            let a_is_dir = a.path().is_dir();
            let b_is_dir = b.path().is_dir();
            if a_is_dir != b_is_dir { b_is_dir.cmp(&a_is_dir) } 
            else { a.file_name().cmp(&b.file_name()) }
        });

        for entry in entries {
            let file_path = entry.path();
            let file_name = entry.file_name().to_string_lossy().to_string();

            if file_path.is_dir() {
                ui.collapsing(format!("📁 {}", file_name), |ui| {
                    render_tree(ui, &file_path, app);
                });
            } else {
                let icon = if file_name.ends_with(".rs") { "🦀" } else { "📄" };
                if ui.selectable_label(false, format!("{} {}", icon, file_name)).clicked() {
                    app.open_file_from_path(file_path);
                }
            }
        }
    }
}