use std::time::Duration;

use chrono::Local;
use eframe::egui;
use egui::{Align, Color32, CornerRadius, Frame, Layout, Margin, Sense, Stroke, UiBuilder};

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "Miata Dash",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

struct MyApp {
    style_applied: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            style_applied: false,
        }
    }
}

#[derive(Clone, Copy)]
struct Palette {
    background: Color32,
    foreground: Color32,
    card: Color32,
    card_hover: Color32,
    muted: Color32,
    border: Color32,
}

impl Palette {
    fn dark() -> Self {
        Self {
            background: Color32::from_rgb(18, 18, 22),
            foreground: Color32::from_rgb(238, 238, 244),
            card: Color32::from_rgb(28, 28, 34),
            card_hover: Color32::from_rgb(36, 36, 44),
            muted: Color32::from_rgb(160, 160, 176),
            border: Color32::from_rgba_premultiplied(90, 90, 104, 120),
        }
    }
    fn light() -> Self {
        Self {
            background: Color32::from_rgb(242, 242, 245),
            foreground: Color32::from_rgb(28, 28, 34),
            card: Color32::from_rgb(255, 255, 255),
            card_hover: Color32::from_rgb(245, 245, 245),
            muted: Color32::from_rgb(120, 120, 136),
            border: Color32::from_rgba_premultiplied(90, 90, 104, 120),
        }
    }
}

struct AppButton {
    id: &'static str,
    name: &'static str,
    glyph: &'static str,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if !self.style_applied {
            apply_style(ctx, Palette::dark());
            self.style_applied = true;
        }

        ctx.request_repaint_after(Duration::from_millis(500));

        let palette = Palette::dark();
        let now = Local::now();
        let time_text = now.format("%H:%M").to_string();

        let apps = [
            AppButton {
                id: "androidauto",
                name: "Android Auto",
                glyph: "AA",
            },
            AppButton {
                id: "music",
                name: "Music",
                glyph: "MS",
            },
            AppButton {
                id: "maps",
                name: "Maps",
                glyph: "MAP",
            },
            AppButton {
                id: "settings",
                name: "Settings",
                glyph: "SET",
            },
        ];

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.set_width(ui.available_width());

            ui.vertical_centered(|ui| {
                ui.add_space(8.0);
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(time_text)
                            .size(64.0)
                            .color(palette.foreground),
                    )
                    .selectable(false),
                );
            });

            ui.add_space(24.0);

            let columns = 5usize;
            let tile_size = egui::vec2(140.0, 140.0);
            for chunk in apps.chunks(columns) {
                ui.horizontal(|ui| {
                    for app in chunk {
                        let response = app_tile(ui, palette, app, tile_size);
                        if response.clicked() {
                            println!("Pressed: {}, ID: {}", app.name, app.id);
                        }
                    }
                });
                ui.add_space(8.0);
            }

            ui.add_space(12.0);
        });
    }
}

fn app_tile(
    ui: &mut egui::Ui,
    palette: Palette,
    app: &AppButton,
    size: egui::Vec2,
) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(size, Sense::click());
    let bg = if response.hovered() {
        palette.card_hover
    } else {
        palette.card
    };

    if ui.is_rect_visible(rect) {
        let mut child = ui.new_child(
            UiBuilder::new()
                .max_rect(rect)
                .layout(Layout::top_down(Align::Center)),
        );
        Frame::new()
            .fill(bg)
            .stroke(Stroke::new(1.0, palette.border))
            .corner_radius(CornerRadius::same(18))
            .inner_margin(Margin::symmetric(16, 14))
            .show(&mut child, |ui| {
                ui.with_layout(Layout::top_down(Align::Center), |ui| {
                    ui.add_space(4.0);
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(app.glyph)
                                .size(36.0)
                                .color(palette.muted),
                        )
                        .selectable(false),
                    );
                    ui.add_space(10.0);
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(app.name)
                                .size(16.0)
                                .color(palette.foreground),
                        )
                        .selectable(false),
                    );
                });
            });
    }

    response
}

fn apply_style(ctx: &egui::Context, palette: Palette) {
    let mut visuals = egui::Visuals::dark();
    visuals.override_text_color = Some(palette.foreground);
    visuals.widgets.noninteractive.bg_fill = palette.background;
    visuals.widgets.noninteractive.fg_stroke.color = palette.foreground;
    visuals.widgets.inactive.bg_fill = palette.card;
    visuals.widgets.inactive.fg_stroke.color = palette.foreground;
    visuals.widgets.hovered.bg_fill = palette.card_hover;
    visuals.widgets.hovered.fg_stroke.color = palette.foreground;
    visuals.widgets.active.bg_fill = palette.card_hover;
    visuals.widgets.active.fg_stroke.color = palette.foreground;
    visuals.widgets.open.bg_fill = palette.card;
    visuals.window_fill = palette.background;
    visuals.panel_fill = palette.background;

    let mut style = (*ctx.style()).clone();
    style.visuals = visuals;
    style.spacing.item_spacing = egui::vec2(14.0, 14.0);
    style.spacing.button_padding = egui::vec2(14.0, 12.0);
    style.spacing.window_margin = Margin::same(14);
    style.visuals.widgets.noninteractive.corner_radius = CornerRadius::same(18);
    style.visuals.widgets.inactive.corner_radius = CornerRadius::same(18);
    style.visuals.widgets.hovered.corner_radius = CornerRadius::same(18);
    style.visuals.widgets.active.corner_radius = CornerRadius::same(18);

    ctx.set_style(style);
}
