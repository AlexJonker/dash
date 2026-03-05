use chrono::Local;
use eframe::egui;
use egui::{Align, CornerRadius, Frame, Layout, Margin, Sense, Stroke, UiBuilder};

use crate::theme::Palette;

pub enum HomeAction {
    OpenSettings,
}

struct AppButton {
    id: &'static str,
    name: &'static str,
    glyph: &'static str,
}

pub fn show(ctx: &egui::Context, palette: Palette) -> Option<HomeAction> {
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

    let mut open_settings = false;

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
                        if app.id == "settings" {
                            open_settings = true;
                        } else {
                            println!("Pressed: {}, ID: {}", app.name, app.id);
                        }
                    }
                }
            });
            ui.add_space(8.0);
        }

        ui.add_space(12.0);
    });

    if open_settings {
        Some(HomeAction::OpenSettings)
    } else {
        None
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
