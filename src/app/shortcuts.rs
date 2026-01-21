use eframe::egui;
use crate::panels::top::TopBarCommand;

pub fn handle(ctx: &egui::Context) -> Option<TopBarCommand> {
    ctx.input(|i| {
        if i.key_pressed(egui::Key::N) && i.modifiers.ctrl {
            return Some(TopBarCommand::NewFile);
        }
        if i.key_pressed(egui::Key::S) && i.modifiers.ctrl {
            return Some(TopBarCommand::SaveFile);
        }
        if i.key_pressed(egui::Key::W) && i.modifiers.ctrl {
            return Some(TopBarCommand::CloseTab);
        }
        if i.key_pressed(egui::Key::O) && i.modifiers.ctrl {
            // Ctrl + Shift + O for Folders
            if i.modifiers.shift {
                return Some(TopBarCommand::OpenFolder);
            }
            // Ctrl + O for Files
            return Some(TopBarCommand::OpenFile);
        }

        None
    })
}