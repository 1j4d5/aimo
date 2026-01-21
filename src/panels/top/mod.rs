use eframe::egui;
pub mod titlebar;
pub mod menu;

#[derive(Debug, Clone, Copy)]
pub enum TopBarCommand {
    NewFile,
    OpenFile,
    OpenFolder,
    SaveFile,
    CloseTab,
    EditTheme,
    ReloadTheme,
}

pub struct TopBarState {
    pub is_maximized: bool,
    pub target_pos: Option<egui::Pos2>,
    pub current_pos: Option<egui::Pos2>,
}

impl Default for TopBarState {
    fn default() -> Self {
        Self { 
            is_maximized: false,
            target_pos: None,
            current_pos: None,
        }
    }
}

pub fn show(ctx: &egui::Context, state: &mut TopBarState) -> Option<TopBarCommand> {
    let mut command = None;

    egui::TopBottomPanel::top("top_bar")
        .exact_height(32.0)
        .frame(egui::Frame::default()
            .fill(egui::Color32::from_rgb(30, 30, 30))
            .inner_margin(0)
        )
        .show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                // Left side: Traffic lights and Title
                titlebar::handle_dragging(ui, ctx, state);

                // Right side: Menus
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add_space(10.0); 
                    command = menu::show_menus(ui, ctx);
                });
            });
        });

    command
}