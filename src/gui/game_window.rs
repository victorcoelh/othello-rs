use eframe::egui::{self, Color32, Stroke};

pub fn build_game_window() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 800.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Ferris Othello",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<MyApp>::default())
        }))
}

#[derive(Default)]
struct MyApp {}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Game Board");

            if ui.button("Mojo Pin").clicked() {
                println!("uhhhh uhhhhu uuuuuhhhhhh")
            }
            
            let painter = ui.painter();
            painter.circle(
                egui::Pos2{x: 20.0, y: 20.0},
                50.0,
                Color32::BLUE,
                Stroke{width: 2.0, color: Color32::from_rgb(255, 255, 255)})
        });
    }
}
