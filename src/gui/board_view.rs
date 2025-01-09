use core::f32;

use eframe::egui::{self, Color32, Layout, Ui, Vec2};

use crate::game_controller::GameController;

static BORDER_COLOR: Color32 = Color32::from_rgb(0x54, 0x77, 0x35);
static BOARD_COLOR: Color32 = Color32::from_rgb(0x26, 0x70, 0x39);
static BUTTON_COLOR: Color32 = Color32::from_rgb(0xFF, 0x5A, 0x36);
static BLACK_PIECE: egui::ImageSource = egui::include_image!("../../assets/black_piece.png");
static WHITE_PIECE: egui::ImageSource = egui::include_image!("../../assets/white_piece.png");

pub struct BoardView {
    chatbox_text: String,
    text_font: egui::FontId,
    rank_font: egui::FontId,
}

impl BoardView {
    pub fn new() -> Self {
        BoardView {
            chatbox_text: String::new(),
            text_font: egui::FontId::proportional(16.0),
            rank_font: egui::FontId::monospace(18.0),
         }
    }

    pub fn draw(&mut self, ctx: &egui::Context, controller: &mut GameController) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                let turn_text = match controller.player_turn {
                    true => "Your turn!",
                    false => "Waiting for opponent..."
                };

                ui.heading(turn_text);
                ui.add_space(50.0);

                ui.horizontal_top(|ui| {
                    ui.set_min_width(ui.available_width());
                    ui.add_space((ui.available_width() / 2.0) - ((8.0*48.0)/2.0) - 45.0);
                    self.board_widget(ui, controller);

                    ui.vertical_centered(|ui| {
                        let (yours, opponents) = match controller.is_host {
                            true => (BLACK_PIECE.clone(), WHITE_PIECE.clone()),
                            false => (WHITE_PIECE.clone(), BLACK_PIECE.clone()),
                        };

                        ui.heading("yours");
                        ui.add_sized([80.0, 80.0], egui::Image::new(yours));
                        ui.heading("opponent");
                        ui.add_sized([80.0, 80.0], egui::Image::new(opponents));
                    });
                });

                ui.add_space(50.0);
                ui.with_layout(Layout::right_to_left(egui::Align::BOTTOM), |ui| {
                    self.menu_widget(ui, controller);
                    self.chat_widget(ui, controller);
                });
            });
        });
    }

    fn board_widget(&mut self, ui: &mut Ui, controller: &mut GameController) {
        egui::Frame::none()
            .fill(BOARD_COLOR)
            .rounding(5.0)
            .inner_margin(10.0)
            .multiply_with_opacity(0.75)
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.add_space(40.0);

                        for i in 0..8 {
                            let character = (('A' as u8) + i) as char;
                            let text = egui::RichText::new(character)
                                .font(self.rank_font.clone())
                                .color(Color32::WHITE);

                            ui.add_space(12.5);
                            ui.heading(text);
                            ui.add_space(17.0);
                        }
                    });
                    ui.add_space(10.0);

                    ui.horizontal_top(|ui| {
                        ui.add_space(8.0);
                        ui.vertical(|ui| {
                            for i in 0..8 {
                                let text = egui::RichText::new(i.to_string())
                                    .font(self.rank_font.clone())
                                    .color(Color32::WHITE);
    
                                ui.add_space(12.5);
                                ui.heading(text);
                                ui.add_space(13.0);
                            }
                        });
                        ui.add_space(10.0);
    
                        egui::Grid::new("board")
                            .max_col_width(48.0)
                            .min_col_width(48.0)
                            .min_row_height(48.0)
                            .spacing(Vec2::new(0.0, 1.5))
                            .with_row_color(|_, _| Some(BOARD_COLOR.clone()))
                            .show(ui, |ui| {
                                for i in 0..8 {
                                    for j in 0..8 {
                                        self.cell_widget(ui, i, j, controller);
                                    }
                                    ui.end_row();
                                }
                            });
                    });
                });
            });
    }

    fn cell_widget(&mut self, ui: &mut Ui, i: usize, j: usize, controller: &mut GameController) {
        egui::Frame::none()
            .inner_margin(0.0)
            .outer_margin(0.0)
            .stroke(egui::Stroke::new(1.0, Color32::BLACK))
            .show(ui, |ui| {
                ui.set_min_height(ui.available_height());
                ui.set_min_width(ui.available_width());
                ui.horizontal_centered(|ui| {
                    ui.vertical_centered_justified(|ui| {
                        if let Some(piece) = controller.get_piece_at(i, j) {
                            let image = match piece {
                                0 => BLACK_PIECE.clone(),
                                1 => WHITE_PIECE.clone(),
                                _ => panic!("Invalid piece type at position {i}:{j}")
                            };
                            ui.add(egui::Image::new(image).sense(egui::Sense::click()));
                        } else {
                            let button = ui.add(egui::Button::new("")
                                .frame(false)
                                .min_size(ui.available_size()));

                            if button.clicked() {
                                controller.try_set_piece_on_board(i, j, false);
                            }
                        }
                    })
                });
            });
    }

    fn chat_widget(&mut self, ui: &mut Ui, controller: &mut GameController) {
        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            egui::Frame::none()
                .inner_margin(10.0)
                .rounding(5.0)
                .fill(Color32::BLACK)
                .stroke(egui::Stroke::new(8.0, BORDER_COLOR.clone()))
                .multiply_with_opacity(0.7)
                .show(ui,|ui| {
                    let textbox = ui.add(egui::TextEdit::singleline(&mut self.chatbox_text)
                        .font(self.text_font.clone())
                        .hint_text("write your message here!")
                        .desired_width(f32::INFINITY));

                    if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        if !self.chatbox_text.is_empty(){
                            controller.push_chat_message(self.chatbox_text.clone(), false);

                            self.chatbox_text.clear();
                            ui.memory_mut(|mem| {
                                mem.request_focus(textbox.id);
                            });
                        }
                    }

                    egui::ScrollArea::vertical()
                        .max_width(f32::INFINITY)
                        .show(ui, |ui| {
                            ui.with_layout(Layout::top_down_justified(egui::Align::LEFT), |ui| {
                                for text in controller.get_chat_messages() {
                                    let color = if text.contains("WARNING:") {
                                        Color32::YELLOW
                                    } else if text.contains("ERROR:") {
                                        Color32::LIGHT_RED
                                    } else {
                                        Color32::WHITE
                                    };

                                    ui.label(egui::RichText::new(text)
                                        .color(color)
                                        .font(self.text_font.clone()));

                                    ui.add_space(1.0);
                                };
                            });
                        });
                });
        });
    }

    fn menu_widget(&mut self, ui: &mut Ui, controller: &mut GameController) {
        egui::Frame::none()
            .rounding(5.0)
            .inner_margin(5.0)
            .stroke(egui::Stroke::new(8.0, BORDER_COLOR.clone()))
            .fill(Color32::BLACK)
            .multiply_with_opacity(0.7)
            .show(ui, |ui| {
                ui.set_max_width(300.0);
                ui.vertical_centered_justified(|ui| {
                    if self.button_widget(ui, "Undo Last Move").clicked() {
                        controller.undo_last_move();
                    }

                    if self.button_widget(ui, "Surrender").clicked() {
                        controller.surrender();
                    }

                    if self.button_widget(ui, "Pass Turn").clicked() {
                        controller.try_pass_turn()
                    }
                });
            });
    }

    fn button_widget(&mut self, ui: &mut Ui, text: &str) -> egui::Response {
        let text = egui::RichText::new(text)
            .color(Color32::WHITE)
            .font(self.text_font.clone());

        return ui.add(egui::Button::new(text)
            .fill(BUTTON_COLOR.clone())
            .rounding(5.0)
            .min_size(Vec2::new(0.0, 50.0)))
    }
}
