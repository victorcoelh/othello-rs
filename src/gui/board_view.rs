use eframe::egui;

use crate::game_controller::GameController;


pub fn board_view(ctx: &eframe::egui::Context, _controller: &mut GameController) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.heading("Connected!")
        });
    });
}
