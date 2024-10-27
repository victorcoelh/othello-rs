use core::f32;

use eframe::egui::{self, Color32, Layout, Ui, Vec2};

use crate::game_controller::GameController;

static BORDER_COLOR: Color32 = Color32::from_rgb(0x54, 0x77, 0x35);
static BOARD_COLOR: Color32 = Color32::from_rgb(0x26, 0x70, 0x39);
static BUTTON_COLOR: Color32 = Color32::from_rgb(0xFF, 0x5A, 0x36);

pub struct BoardView {
    chatbox_text: String,
    text_font: egui::FontId,
    rank_font: egui::FontId,
}

impl BoardView {
    pub fn new() -> Self {
        let mut board = [[None; 8]; 8];
        board[3][3] = Some(true);
        board[3][4] = Some(false);
        board[4][3] = Some(false);
        board[4][4] = Some(true);

        BoardView {
            chatbox_text: String::new(),
            text_font: egui::FontId::proportional(16.0),
            rank_font: egui::FontId::monospace(18.0),
         }
    }

    pub fn draw(&mut self, ctx: &egui::Context, controller: &mut GameController) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Connected!");
                ui.add_space(50.0);

                self.board_widget(ui, controller);
                ui.add_space(100.0);

                ui.with_layout(Layout::right_to_left(egui::Align::BOTTOM), |ui| {
                    self.menu_widget(ui);
                    self.chat_widget(ui, controller);
                });
            });
        });
    }

    fn board_widget(&mut self, ui: &mut Ui, controller: &mut GameController) {
        ui.horizontal_top(|ui| {
            ui.set_min_width(ui.available_width());
            ui.add_space((ui.available_width() / 2.0) - ((8.0*48.0)/2.0) - 45.0);

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
                                0 => egui::include_image!("../../assets/black_piece.png"),
                                1 => egui::include_image!("../../assets/white_piece.png"),
                                _ => panic!("Invalid piece type at position {i}:{j}")
                            };

                            ui.add(egui::Image::new(image));
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
                            controller.push_message(format!(
                                "player: {}", self.chatbox_text.clone())
                            );

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
                                for text in controller.get_messages() {
                                    ui.label(egui::RichText::new(text)
                                        .color(Color32::WHITE)
                                        .font(self.text_font.clone()));

                                    ui.add_space(1.0);
                                };
                            });
                        });
                });
        });
    }

    fn menu_widget(&mut self, ui: &mut Ui) {
        egui::Frame::none()
            .rounding(5.0)
            .inner_margin(5.0)
            .stroke(egui::Stroke::new(8.0, BORDER_COLOR.clone()))
            .fill(Color32::BLACK)
            .multiply_with_opacity(0.7)
            .show(ui, |ui| {
                ui.set_max_width(300.0);
                ui.vertical_centered_justified(|ui| {
                    if self.button_widget(ui, "Undo Move").clicked() {
                        println!("Undo");
                    }

                    if self.button_widget(ui, "Surrender").clicked() {
                        println!("Surrender");
                    }

                    if self.button_widget(ui, "Settings").clicked() {
                        println!("Settings");
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
