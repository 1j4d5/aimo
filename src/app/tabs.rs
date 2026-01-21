use eframe::egui;
use crate::app::state::EditorApp;

pub fn show_tab_bar(app: &mut EditorApp, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 4.0;
        let mut to_remove = None;

        for (i, buffer) in app.buffers.iter().enumerate() {
            let is_active = app.active_buffer_idx == i;
            ui.group(|ui| {
                let text = egui::RichText::new(&buffer.name)
                    .color(if is_active { 
                        app.theme.col(app.theme.keywords) 
                    } else { 
                        app.theme.col(app.theme.text) 
                    });

                if ui.selectable_label(is_active, text).clicked() {
                    app.active_buffer_idx = i;
                }
                if ui.small_button("x").clicked() {
                    to_remove = Some(i);
                }
            });
        }

        if let Some(idx) = to_remove {
            app.close_buffer(idx);
        }
    });
}