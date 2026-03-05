use std::time::Duration;

use eframe::egui;

use crate::pages::Controller;
use crate::theme::apply_style;

pub fn run() -> Result<(), eframe::Error> {
    eframe::run_native(
        "Miata Dash",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<DashApp>::default())),
    )
}

struct DashApp {
    controller: Controller,
    style_applied: bool,
}

impl Default for DashApp {
    fn default() -> Self {
        Self {
            controller: Controller::new(),
            style_applied: false,
        }
    }
}

impl DashApp {
    fn ensure_style(&mut self, ctx: &egui::Context) {
        let palette = self.controller.palette();
        if !self.style_applied || self.controller.style_needs_refresh() {
            apply_style(ctx, palette);
            self.controller.mark_style_applied();
            self.style_applied = true;
        }
    }
}

impl eframe::App for DashApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.ensure_style(ctx);
        self.controller.update(ctx);
        self.ensure_style(ctx);

        ctx.request_repaint_after(Duration::from_millis(500));
    }
}
