use eframe::egui;
use super::TopBarCommand;

pub fn show_menus(ui: &mut egui::Ui, ctx: &egui::Context) -> Option<TopBarCommand> {
    let mut command = None;
    let menu_names = ["Env", "Edit", "Terminal", "Build"];
    ui.add_space(10.0);

    for name in menu_names {
        let id = ui.make_persistent_id(name);
        let (rect, response) = ui.allocate_at_least(egui::Vec2::new(70.0, 24.0), egui::Sense::hover());

        if response.hovered() {
            egui::Popup::open_id(ctx, id);
        }

        let is_open = ui.memory(|mem| mem.is_popup_open(id));
        if is_open {
            ui.painter().rect_filled(rect.expand(2.0), egui::CornerRadius::same(6), egui::Color32::from_rgb(210, 210, 210));
        }

        ui.painter().text(rect.center(), egui::Align2::CENTER_CENTER, name, egui::TextStyle::Body.resolve(ui.style()), egui::Color32::BLACK);

        egui::popup_below_widget(ui, id, &response, egui::PopupCloseBehavior::CloseOnClick, |ui| {
            ui.set_min_width(160.0);
            match name {
                "Env" => {
                    if ui.button("📄 New File").clicked() { command = Some(TopBarCommand::NewFile); }
                    if ui.button("📂 Open File").clicked() { command = Some(TopBarCommand::OpenFile); }
                    if ui.button("📁 Open Folder").clicked() { command = Some(TopBarCommand::OpenFolder); }
                    ui.separator();
                    if ui.button("💾 Save File").clicked() { command = Some(TopBarCommand::SaveFile); }
                    if ui.button("❌ Close Tab").clicked() { command = Some(TopBarCommand::CloseTab); }
                }
                "Edit" => {
                    if ui.button("🎨 Edit Theme (color.json)").clicked() { command = Some(TopBarCommand::EditTheme); }
                    if ui.button("🔄 Reload Theme").clicked() { command = Some(TopBarCommand::ReloadTheme); }
                }
                _ => { ui.label("Coming soon..."); }
            }
        });
    }
    command
}