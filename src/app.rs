use crate::geometry::scara::{Scara, ScaraState};

pub struct ScaraApp {
    robot: Scara,
    state: ScaraState,
    sim_time: f64,
    dt: f64,
    tracking_error_rms: f64,
    target: [f64; 2],
}

impl ScaraApp {
    pub fn new() -> Self {
        Self {
            robot: Scara{
                l1: 50.0,
                l2: 50.0,
                base: [0.0, 0.0],
            },
            state: ScaraState{
                theta1: 0.5,
                theta2: 0.3,
            },
            sim_time: 0.0,
            dt: 0.01,
            tracking_error_rms: 0.0,
            target: [0.0, 0.0],
        }
    }
}

impl eframe::App for ScaraApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // =========================
        // LEFT SIDEBAR
        // =========================
        egui::SidePanel::left("left_panel")
            .resizable(true)
            .default_width(260.0)
            .show(ctx, |ui| {

                ui.heading("TARGET LOCATION");
                ui.separator();

                let max:f64 = self.robot.l1 + self.robot.l2;

                ui.add(
                    egui::Slider::new(&mut self.target[0], -max..=max)
                        .text("x"),
                );

                ui.add(
                    egui::Slider::new(&mut self.target[1], -max..=max)
                        .text("y"),
                );

                ui.separator();
                ui.heading("ROBOT GEOMETRY");

                ui.add(
                    egui::Slider::new(&mut self.robot.l1, 50.0..=300.0)
                        .text("Link 1"),
                );

                ui.add(
                    egui::Slider::new(&mut self.robot.l2, 50.0..=300.0)
                        .text("Link 2"),
                );

                ui.with_layout(
                    egui::Layout::bottom_up(egui::Align::LEFT),
                    |ui| {
                        ui.separator();
                        ui.label(format!("dt: {:.4} s", self.dt));
                        ui.label(format!("Sim Time: {:.3} s", self.sim_time));
                        ui.label(format!("RMS Error: {:.6}", self.tracking_error_rms));
                    },
                );
            });



        // =========================
        // CENTRAL ROBOT VIEW
        // =========================
        
        egui::CentralPanel::default().show(ctx, |ui| {

            let available = ui.available_size();
            let (response, painter) =
                ui.allocate_painter(available, egui::Sense::hover());

            let rect = response.rect;
            let center = rect.center();

            // --- Dynamic scaling ---
            let world_radius = self.robot.l1 + self.robot.l2;
            let min_dim = rect.width().min(rect.height());
            let scale = 0.45 * min_dim / world_radius as f32;

            // --- Inverse kinematics ---
            let done = self.robot.ik(self.target, &mut self.state);

            if !done {
                //println!("Inverse kinematics error");
            }

            // --- Forward kinematics ---
            let max:f32 = ((self.robot.l1 + self.robot.l2) as f32)*scale;
            let (joint1, end_eff) = self.robot.fk(&self.state);
            //println!("joint1: ({:.1}, {:.1}), end_eff: ({:.1}, {:.1})",joint1[0], joint1[1],end_eff[0], end_eff[1]);
            let base_screen = world_to_screen(self.robot.base, center, scale);
            let joint1_screen = world_to_screen(joint1, center, scale);
            let end_screen = world_to_screen(end_eff, center, scale);
            let target_screen = world_to_screen(self.target, center, scale);

            // --- Draw Reach Area ---
            painter.circle_filled(base_screen,max, egui::Color32::from_rgb(55, 55, 55));

            // --- Draw links ---
            painter.line_segment(
                [base_screen, joint1_screen],
                egui::Stroke::new(4.0, egui::Color32::LIGHT_BLUE),
            );

            painter.line_segment(
                [joint1_screen, end_screen],
                egui::Stroke::new(4.0, egui::Color32::LIGHT_GREEN),
            );

            // --- Draw joints ---
            painter.circle_filled(base_screen, 6.0, egui::Color32::RED);
            painter.circle_filled(joint1_screen, 6.0, egui::Color32::RED);
            painter.circle_filled(end_screen, 5.0, egui::Color32::YELLOW);
            
            // --- Draw target ---
            painter.circle_filled(target_screen, 10.0, egui::Color32::GREEN);
        });

        self.sim_time += self.dt;
        ctx.request_repaint();

    }
}

fn world_to_screen(
    world: [f64; 2],
    center: egui::Pos2,
    scale: f32,
) -> egui::Pos2 {
    egui::pos2(
        center.x + (world[0] as f32 * scale),
        center.y - (world[1] as f32 * scale),
    )
}
