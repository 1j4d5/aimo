use eframe::egui;
use super::TopBarCommand;

pub fn show_menus(ui: &mut egui::Ui, _ctx: &egui::Context) -> Option<TopBarCommand> {
    let mut command = None;
    let menu_names = ["Build", "Terminal", "Edit", "Env"]; 

    for name in menu_names {
        // Standard egui menu buttons handle state automatically
        ui.menu_button(name, |ui| {
            ui.set_min_width(160.0);
            match name {
                "Env" => {
                    if ui.button("📄 New File").clicked() { command = Some(TopBarCommand::NewFile); ui.close_menu(); }
                    if ui.button("📂 Open File").clicked() { command = Some(TopBarCommand::OpenFile); ui.close_menu(); }
                    if ui.button("📁 Open Folder").clicked() { command = Some(TopBarCommand::OpenFolder); ui.close_menu(); }
                    ui.separator();
                    if ui.button("💾 Save File").clicked() { command = Some(TopBarCommand::SaveFile); ui.close_menu(); }
                    if ui.button("❌ Close Tab").clicked() { command = Some(TopBarCommand::CloseTab); ui.close_menu(); }
                }
                "Edit" => {
                    if ui.button("🎨 Edit Theme").clicked() { command = Some(TopBarCommand::EditTheme); ui.close_menu(); }
                    if ui.button("🔄 Reload Theme").clicked() { command = Some(TopBarCommand::ReloadTheme); ui.close_menu(); }
                }
                _ => { 
                    ui.label(format!("{} tools coming soon", name)); 
                }
            }
        });
        ui.add_space(4.0);
    }

    command
}