use eframe::egui::{self, Ui};

use crate::game_controller::GameController;


pub fn board_view(ctx: &eframe::egui::Context, controller: &mut GameController) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.heading("Connected!");
            ui.add_space(100.0);

            chat_widget(ui, controller.get_messages());
        });
    });
}

fn chat_widget(ui: &mut Ui, chat_messages: &Vec<String>) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        for text in chat_messages {
            let mut text = text.as_str();
            ui.text_edit_singleline(&mut text);
        }
    });
}
