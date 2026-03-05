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
}

fn icons_source(app_id: &str) -> egui::ImageSource<'static> {
    match app_id {
        "androidauto" => egui::include_image!("../../../assets/icons/android_auto.svg"),
        "music" => egui::include_image!("../../../assets/icons/music.svg"),
        "maps" => egui::include_image!("../../../assets/icons/maps.svg"),
        "settings" => egui::include_image!("../../../assets/icons/settings.svg"),
        _ => egui::include_image!("../../../assets/icons/none.svg"),
    }
}


pub fn show(ctx: &egui::Context, palette: Palette) -> Option<HomeAction> {
    let now = Local::now();
    let time_text = now.format("%H:%M").to_string();

    let apps = [
        AppButton {
            id: "androidauto",
            name: "Android Auto",
        },
        AppButton {
            id: "music",
            name: "Music",
        },
        AppButton {
            id: "maps",
            name: "Maps",
        },
        AppButton {
            id: "settings",
            name: "Settings",
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
                let spacing = ui.spacing().item_spacing.x;
                let row_width = chunk.len() as f32 * tile_size.x
                    + (chunk.len().saturating_sub(1) as f32) * spacing;
                let pad = ((ui.available_width() - row_width) / 2.0).max(0.0);

                ui.add_space(pad);

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
                        egui::Image::new(icons_source(app.id))
                            .fit_to_exact_size(egui::vec2(38.0, 38.0))
                            .tint(palette.foreground),
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