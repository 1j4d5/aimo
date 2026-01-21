use eframe::egui;
use crate::code::theme::EditorTheme;

pub fn show(ui: &mut egui::Ui, text: &mut String, theme: &EditorTheme) {
    let mut layouter = |ui: &egui::Ui, buffer: &dyn egui::TextBuffer, wrap_width: f32| {
        let string = buffer.as_str();
        let mut job = egui::text::LayoutJob::default();
        let font_id = egui::TextStyle::Monospace.resolve(ui.style());

        for word in string.split_inclusive(|c: char| !c.is_alphanumeric()) {
            let color = match word.trim() {
                "fn" | "let" | "mut" | "pub" | "use" | "struct" | "impl" | "match" | "return" => {
                    theme.col(theme.keywords)
                }
                "String" | "u32" | "i32" | "bool" | "f32" | "Option" | "Self" => {
                    theme.col(theme.types)
                }
                _ if word.trim().starts_with('"') || word.trim().ends_with('"') => {
                    theme.col(theme.strings)
                }
                _ if word.trim().starts_with("//") => theme.col(theme.comments),
                _ => theme.col(theme.text),
            };

            job.append(word, 0.0, egui::TextFormat {
                font_id: font_id.clone(),
                color,
                ..Default::default()
            });
        }

        job.wrap.max_width = wrap_width;
        ui.fonts_mut(|f| f.layout_job(job))
    };

    ui.horizontal_top(|ui| {
        let line_count = text.lines().count().max(1);
        let mut line_nums = String::new();
        for i in 1..=line_count {
            line_nums.push_str(&format!("{:>3}\n", i));
        }

        ui.add_space(5.0);
        ui.label(
            egui::RichText::new(line_nums)
                .font(egui::TextStyle::Monospace.resolve(ui.style()))
                .color(theme.col(theme.comments))
        );
        
        ui.add_space(10.0);

        ui.add(
            egui::TextEdit::multiline(text)
                .font(egui::TextStyle::Monospace)
                .desired_width(f32::INFINITY)
                .lock_focus(true)
                .layouter(&mut layouter)
                .frame(false),
        );
    });
}