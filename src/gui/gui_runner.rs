use std::sync::{Arc, Mutex};

use eframe::egui;

use super::{board_view::BoardView, main_menu_view::MainMenuView, game_end_view::GameEndView};
use crate::game_controller::{GameController, GameState};


pub fn build_game_window(controller: Arc<Mutex<GameController>>) -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([850.0, 850.0]),
        ..Default::default()
    };
    let menu = GuiRunner::new(controller);

    eframe::run_native(
        "Ferris Othello",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(menu))
        })
    )
}

struct GuiRunner {
    controller: Arc<Mutex<GameController>>,
    error: Option<String>,
    board_view: BoardView,
    main_menu_view: MainMenuView,
    game_end_view: GameEndView
}

impl eframe::App for GuiRunner {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        let mut controller = self.controller.lock().unwrap();
        match controller.state {
            GameState::NoConnection => {
                self.main_menu_view.draw(ctx, &mut controller);
            },
            GameState::Playing => self.board_view.draw(ctx, &mut controller),
            GameState::GameEnded(player_won) => {
                let player_won = player_won.clone();
                self.game_end_view.draw(ctx, &mut controller, player_won)
            }
        };

        if let Some(error) = controller.error_queue.lock().unwrap().pop() {
            self.error = Some(error);
        }

        drop(controller);

        if let Some(error) = self.error.clone() {
            self.error_window(ctx, error);
        }
    }
}

impl GuiRunner {
    fn new(controller: Arc<Mutex<GameController>>) -> Self {
        GuiRunner {
            controller: controller,
            error: None,
            board_view: BoardView::new(),
            main_menu_view: MainMenuView::new(),
            game_end_view: GameEndView::new()
        }
    }

    fn error_window(&mut self, ctx: &egui::Context, error: String) {
        egui::Window::new("Connection Error").show(ctx, |ui| {
            ui.heading(error);
            ui.add_space(10.0);

            if ui.button("Go Back to Menu").clicked() {
                self.error = None;
                self.controller.lock().unwrap().restart_game();
            }

            if ui.button("Ok").clicked() {
                self.error = None;
            }
        });
    }
}
