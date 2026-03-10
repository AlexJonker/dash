use std::time::Duration;

use eframe::egui;
use egui::{FontData, FontDefinitions, FontFamily};

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

            // Support Chinese, Japanese and Korean characters (for music)
            let mut fonts = FontDefinitions::default();

            fonts.font_data.insert(
                "noto_cjk".to_owned(),
                FontData::from_static(include_bytes!("../assets/fonts/NotoSansCJK-Regular.ttc"))
                    .into(),
            );

            fonts
                .families
                .get_mut(&FontFamily::Proportional)
                .unwrap()
                .insert(0, "noto_cjk".to_owned());

            cc.egui_ctx.set_fonts(fonts);

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
