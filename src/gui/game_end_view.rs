use eframe::egui::{self, Color32, Vec2};

use crate::game_controller::{GameController, GameResult};

static BUTTON_COLOR: Color32 = Color32::from_rgb(0xFF, 0x5A, 0x36);

pub struct GameEndView {
    text_font: egui::FontId
}

impl GameEndView {
    pub fn new() -> Self {
        GameEndView {
            text_font: egui::FontId::proportional(16.0)
        }
    }

    pub fn draw(&mut self, ctx: &egui::Context, controller: &mut GameController, player_won: GameResult) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(400.0);

                let end_text = match player_won {
                    GameResult::PlayerWon => "You Win! Congratulations",
                    GameResult::PlayerLost => "You lose... better luck next time!",
                    GameResult::Tie => "The game tied. Better luck next time!"
                };

                ui.heading(
                    egui::RichText::new(end_text)
                    .font(self.text_font.clone())
                    .size(24.0)
                );

                let button = ui.add(
                    egui::Button::new("Go Back")
                        .fill(BUTTON_COLOR)
                        .frame(false)
                        .min_size(Vec2::new(100.0, 40.0))
                );

                if button.clicked() {
                    controller.restart_game()
                }
            });
        });
    }
}
