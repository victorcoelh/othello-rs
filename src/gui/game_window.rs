use eframe::egui;

use super::{board_view::board_view, main_menu_view::main_menu_view};
use crate::game_controller::{GameController, GameState};


pub fn build_game_window(controller: GameController) -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 800.0]),
        ..Default::default()
    };
    let menu = GameWindow::new(controller);

    eframe::run_native(
        "Ferris Othello",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(menu))
        }))
}

struct GameWindow {
    controller: GameController,
    ip_addr: String,
}

impl eframe::App for GameWindow {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        match self.controller.get_state() {
            GameState::NoConnection => main_menu_view(ctx, &mut self.controller, &mut self.ip_addr),
            GameState::Playing => board_view(ctx, &mut self.controller),
            GameState::GameEnded => board_view(ctx, &mut self.controller),
        }
    }
}

impl GameWindow {
    fn new(controller: GameController) -> Self {
        GameWindow {
            controller: controller,
            ip_addr: String::new()
        }
    }
}
