use std::sync::mpsc::Sender;

use eframe::egui;

use crate::game_controller::GameController;


pub struct MainMenuView{
    socket_addr: String,
    error: Option<String>,
}

impl MainMenuView {
    pub fn new() -> Self {
        MainMenuView {
            socket_addr: "192.168.56.101:8069".to_string(),
            error: None
        }
    }

    pub fn draw(&mut self, ctx: &egui::Context, controller: &mut GameController, error_tx: Sender<String>){
        self.main_window(ctx, controller, error_tx);

        if let Some(error) = self.error.clone() {
            self.error_window(ctx, &error);
        }
    }

    fn main_window(&mut self, ctx: &egui::Context, controller: &mut GameController, error_tx: Sender<String>) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(300.0);
                ui.heading("Connect to Peer:");
                ui.add_space(20.0);
                ui.text_edit_singleline(&mut self.socket_addr);

                let connect_button = ui.add(
                    egui::Button::new("Connect to Address")
                );

                let wait_button = ui.add(
                    egui::Button::new("Wait for Connection")
                );
        
                if connect_button.clicked() {
                    if let Err(error) = controller.connect(&self.socket_addr, error_tx) {
                        self.error = Some(
                            format!("Error while trying to connect to socket {}: {}",
                            &self.socket_addr, error)
                        )
                    };
                } else {
                    if wait_button.clicked() {
                        if let Err(error) = controller.listen_and_connect(&self.socket_addr, error_tx) {
                            self.error = Some(
                                format!("Error while trying to bind to socket {}: {}",
                                &self.socket_addr, error)
                            )
                        }
                    }
                }
            })
        });
    }

    fn error_window(&mut self, ctx: &egui::Context, error: &String) {
        egui::Window::new("Error").show(ctx, |ui| {
            ui.heading(error);
        });
    }
}
