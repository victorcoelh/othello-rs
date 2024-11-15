use eframe::egui;

use crate::game_controller::GameController;

pub struct GameEndView { }

impl GameEndView {
    pub fn new() -> Self {
        GameEndView { }
    }

    pub fn draw(&mut self, ctx: &egui::Context, _controller: &mut GameController, player_won: bool) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.vertical_centered(|ui| {
                    let end_text = match player_won {
                        true => "You Win!!",
                        false => "You lose... better luck next time!"
                    };

                    ui.heading(end_text);
                    ui.button("Go back")
                });
            });
        });
    }
}
