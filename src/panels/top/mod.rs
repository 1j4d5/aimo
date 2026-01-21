use eframe::egui;

mod titlebar;
mod menu;

#[derive(Default)]
pub struct TopBarState {
    pub target_pos: Option<egui::Pos2>,
    pub current_pos: Option<egui::Pos2>,
}

#[derive(PartialEq, Clone, Copy)]
pub enum TopBarCommand {
    NewFile,
    OpenFile,
    OpenFolder,
    SaveFile,
    CloseTab,
    EditTheme,
    ReloadTheme,
}

pub fn show(ctx: &egui::Context, state: &mut TopBarState) -> Option<TopBarCommand> {
    let mut command = None;
    const TITLEBAR_HEIGHT: f32 = 32.0;

    egui::TopBottomPanel::top("title_bar")
        .exact_height(TITLEBAR_HEIGHT)
        .frame(egui::Frame::none().fill(egui::Color32::from_rgb(236, 236, 236)))
        .show(ctx, |ui| {
            // 1. Handle Window Dragging and Title Text
            titlebar::handle_dragging(ui, ctx, state);

            // 2. Handle Action Menus (Env, Edit, etc.)
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                command = menu::show_menus(ui, ctx);
            });
        });

    command
}