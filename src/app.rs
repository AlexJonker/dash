use std::time::Duration;

use eframe::egui;

use crate::controller::Controller;
use crate::theme::apply_style;

pub fn run() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        renderer: eframe::Renderer::Glow,
        multisampling: 0,
        depth_buffer: 0,
        stencil_buffer: 0,
        viewport: egui::ViewportBuilder::default()
            .with_fullscreen(true)
            .with_decorations(false),
        ..Default::default()
    };

    eframe::run_native(
        "Miata Dash",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            cc.egui_ctx.set_pixels_per_point(1.0);
            Ok(Box::<DashApp>::default())
        }),
    )
}

struct DashApp {
    controller: Controller,
}

impl Default for DashApp {
    fn default() -> Self {
        Self {
            controller: Controller::new(),
        }
    }
}

impl eframe::App for DashApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        apply_style(ctx, self.controller.palette());
        self.controller.update(ctx);

        ctx.request_repaint_after(Duration::from_millis(500));
    }
}
