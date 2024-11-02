use eframe::egui;

use crate::game_controller::GameController;


pub struct MainMenuView{
    socket_addr: String
}

impl MainMenuView {
    pub fn new() -> Self {
        MainMenuView {
            socket_addr: "192.168.56.101:8069".to_string()
        }
    }

    pub fn draw(&mut self, ctx: &egui::Context, controller: &mut GameController) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(300.0);
                ui.heading("Connect to Peer:");
                ui.add_space(20.0);
                ui.text_edit_singleline(&mut self.socket_addr);
        
                if ui.button("Connect to Address").clicked() {
                    controller.connect(&self.socket_addr).unwrap();
                }
        
                if ui.button("Wait for connection").clicked() {
                    controller.listen_and_connect(&self.socket_addr).unwrap();
                }
            })
        });
    }
}
