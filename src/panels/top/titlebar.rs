use eframe::egui;
use super::TopBarState;

pub fn handle_dragging(ui: &mut egui::Ui, ctx: &egui::Context, state: &mut TopBarState) {
    let rect = ui.max_rect();
    let is_maximized = ctx.input(|i| i.viewport().maximized.unwrap_or(false));
    const BUTTON_SIZE: f32 = 12.0;
    const LERP_FACTOR: f32 = 0.20;
    const MIN_SPEED: f32 = 2.0;

    // Drag Interaction
    let bar_response = ui.interact(rect, ui.id().with("bar"), egui::Sense::click_and_drag());
    if bar_response.dragged() && !is_maximized {
        let delta = bar_response.drag_delta();
        if state.target_pos.is_none() {
            let current_win = ctx.input(|i| i.viewport().outer_rect.map(|r| r.min)).unwrap_or(egui::Pos2::ZERO);
            state.target_pos = Some(current_win);
            state.current_pos = Some(current_win);
        }
        if let Some(target) = &mut state.target_pos {
            *target += delta;
        }
    }

    // Smooth Lerp Movement
    if let (Some(current), Some(target)) = (state.current_pos, state.target_pos) {
        let diff = target - current;
        if diff.length() > 0.5 {
            let mut step = diff * LERP_FACTOR;
            if step.length() < MIN_SPEED && diff.length() > MIN_SPEED {
                step = diff.normalized() * MIN_SPEED;
            }
            let next_pos = current + step;
            ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(next_pos));
            state.current_pos = Some(next_pos);
            ctx.request_repaint();
        } else if !bar_response.dragged() {
            state.target_pos = None;
            state.current_pos = None;
        }
    }

    // Render Traffic Light Buttons
    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
        ui.add_space(10.0);
        let buttons = [
            (egui::Color32::from_rgb(255, 95, 87), egui::ViewportCommand::Close),
            (egui::Color32::from_rgb(255, 189, 46), egui::ViewportCommand::Minimized(true)),
            (egui::Color32::from_rgb(39, 201, 63), egui::ViewportCommand::Maximized(!is_maximized)),
        ];

        for (color, cmd) in buttons {
            let (rect, res) = ui.allocate_at_least(egui::Vec2::splat(BUTTON_SIZE), egui::Sense::click());
            let visual_radius = if res.hovered() { BUTTON_SIZE / 1.3 } else { BUTTON_SIZE / 1.5 };
            ui.painter().circle_filled(rect.center(), visual_radius, color);
            if res.clicked() { ctx.send_viewport_cmd(cmd); }
            ui.add_space(2.0);
        }

        ui.add_space(10.0);
        ui.label(egui::RichText::new("Ijaduim IDE @ Arch Linux").color(egui::Color32::BLACK));
    });
}