mod app;
mod panels;
mod code;

use eframe::egui;
// Updated import: EditorApp is now found in the app module
use app::EditorApp; 

fn main() -> eframe::Result<()> {
    // Pre-check theme on boot
    let _ = crate::code::theme::EditorTheme::load();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("AIMO NJIN")
            .with_inner_size([1200.0, 800.0]) // Slightly larger for better IDE feel
            .with_decorations(false)        // Keep false for your custom title bar
            .with_transparent(true),        // Useful for rounded corners or custom UI
        ..Default::default()
    };

    eframe::run_native(
        "Ijaduim",
        options,
        Box::new(|_cc| {
            // Optional: You can customize visual style here via cc.egui_ctx
            Ok(Box::new(EditorApp::default()))
        }),
    )
}