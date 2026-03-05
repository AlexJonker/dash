use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "Miata Dash",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

struct MyApp {
    button_pressed: i32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self { button_pressed: 0 }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Miata Dash");
            ui.separator();

            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing = egui::vec2(20.0, 20.0);

                ui.set_min_size(egui::vec2(120.0, 120.0));
                if ui.button("Android auto").clicked() {
                    self.button_pressed = 1;
                }
                ui.set_min_size(egui::vec2(120.0, 120.0));
                if ui.button("Music").clicked() {
                    self.button_pressed = 2;
                }
                ui.set_min_size(egui::vec2(120.0, 120.0));
                if ui.button("Settings").clicked() {
                    self.button_pressed = 3;
                }
            });

            ui.separator();
            ui.label(format!("Last button pressed: {}", self.button_pressed));
        });
    }
}
