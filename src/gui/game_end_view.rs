use eframe::egui;

use crate::game_controller::GameController;

pub struct GameEndView {
    text_font: egui::FontId
}

impl GameEndView {
    pub fn new() -> Self {
        GameEndView {
            text_font: egui::FontId::proportional(16.0)
        }
    }

    pub fn draw(&mut self, ctx: &egui::Context, controller: &mut GameController, player_won: bool) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.centered_and_justified(|ui| {
                    let end_text = match player_won {
                        true => "You Win!!",
                        false => "You lose... better luck next time!"
                    };

                    ui.heading(
                        egui::RichText::new(end_text)
                        .font(self.text_font.clone())
                        .size(24.0)
                    );

                    let button = ui.button("Go back");
                    if button.clicked() {
                        controller.restart_game()
                    }
                });
            });
        });
    }
}
