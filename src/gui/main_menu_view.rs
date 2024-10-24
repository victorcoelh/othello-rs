use eframe::egui::{self, Color32, Stroke, Ui};

use crate::game_controller::GameController;


pub fn main_menu_view(ctx: &eframe::egui::Context, controller: &mut GameController, menu_text: &mut String) {
    egui::CentralPanel::default().show(ctx, |ui| {
        connection_widget(ui, controller, menu_text);
        
        let painter = ui.painter();
        painter.circle(
            egui::Pos2{x: 100.0, y: 100.0},
            50.0,
            Color32::BLUE,
            Stroke{width: 2.0, color: Color32::from_rgb(255, 255, 255)})
    });
}

fn connection_widget(ui: &mut Ui, controller: &mut GameController, menu_text: &mut String) {
    ui.vertical_centered(|ui| {
        ui.add_space(300.0);
        ui.heading("Connect to Peer:");
        ui.add_space(20.0);
        ui.text_edit_singleline(menu_text);

        if ui.button("Connect to Address").clicked() {
            controller.connect(menu_text).unwrap();
        }

        if ui.button("Wait for connection").clicked() {
            controller.listen_and_connect(menu_text).unwrap();
        }
    });
}
