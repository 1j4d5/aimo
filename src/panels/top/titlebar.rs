use eframe::egui;
use super::TopBarState;

pub fn handle_dragging(ui: &mut egui::Ui, ctx: &egui::Context, state: &mut TopBarState) {
    let rect = ui.max_rect();
    
    // Dynamic centering based on the actual bar height
    let center_y = rect.center().y;
    let mut current_x = rect.min.x + 15.0;

    let base_radius = 6.0;
    let hover_radius = 8.5;
    let spacing = 22.0;
    let hitbox_size = egui::vec2(20.0, 26.0);

    let colors = [
        egui::Color32::from_rgb(255, 95, 87),  // Close
        egui::Color32::from_rgb(255, 189, 46), // Minimize
        egui::Color32::from_rgb(39, 201, 63),  // Maximize
    ];

    for (i, color) in colors.iter().enumerate() {
        let center = egui::pos2(current_x, center_y);
        let interact_rect = egui::Rect::from_center_size(center, hitbox_size);
        
        let response = ui.interact(
            interact_rect, 
            ui.id().with(("traffic", i)), 
            egui::Sense::click()
        );

        let radius = if response.hovered() { hover_radius } else { base_radius };
        ui.painter().circle_filled(center, radius, *color);

        if response.clicked() {
            match i {
                0 => ctx.send_viewport_cmd(egui::ViewportCommand::Close),
                1 => ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true)),
                2 => {
                    state.is_maximized = !state.is_maximized;
                    ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(state.is_maximized));
                }
                _ => {}
            }
        }
        current_x += spacing;
    }

    // Title text aligned to the same center
    let text_pos = egui::pos2(current_x + 5.0, center_y);
    ui.painter().text(
        text_pos,
        egui::Align2::LEFT_CENTER,
        "🚀 Iaduim @ NGIN",
        egui::FontId::proportional(14.0),
        egui::Color32::WHITE,
    );

    // FIX: Drag area stops 250px before the right edge so it doesn't block menus
    let drag_max_x = ui.available_width() - 250.0;
    let drag_rect = egui::Rect::from_min_max(
        egui::pos2(rect.min.x + 80.0, rect.min.y),
        egui::pos2(rect.min.x + drag_max_x, rect.max.y),
    );

    let drag_response = ui.interact(drag_rect, ui.id().with("drag_title"), egui::Sense::drag());
    if drag_response.dragged() {
        ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
    }
}