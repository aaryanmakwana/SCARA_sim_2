use eframe::egui;

mod app;
mod geometry;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 700.0])
            .with_title("SCARA Simulator"),
        ..Default::default()
    };

    eframe::run_native(
        "SCARA Simulator",
        options,
        Box::new(|_cc| {
            Ok(Box::new(app::ScaraApp::new()))
        }),
    )
}
