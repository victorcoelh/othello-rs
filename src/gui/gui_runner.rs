use eframe::egui;

use super::{board_view::BoardView, main_menu_view::MainMenuView, game_end_view::GameEndView};
use crate::game_controller::{GameController, GameState};


pub fn build_game_window(controller: GameController) -> eframe::Result {
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
        }))
}

struct GuiRunner {
    controller: GameController,
    board_view: BoardView,
    main_menu_view: MainMenuView,
    game_end_view: GameEndView
}

impl eframe::App for GuiRunner {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        match self.controller.get_state() {
            GameState::NoConnection => self.main_menu_view.draw(ctx, &mut self.controller),
            GameState::Playing => self.board_view.draw(ctx, &mut self.controller),
            GameState::GameEnded(player_won) => {
                let player_won = *player_won;
                self.game_end_view.draw(ctx, &mut self.controller, player_won)
            }
        }

        self.controller.check_for_new_message();
    }
}

impl GuiRunner {
    fn new(controller: GameController) -> Self {
        GuiRunner {
            controller: controller,
            board_view: BoardView::new(),
            main_menu_view: MainMenuView::new(),
            game_end_view: GameEndView::new()
        }
    }
}
