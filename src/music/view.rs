use eframe::egui;
use egui::{Align, CornerRadius, Frame, Margin, RichText, Stroke};

use crate::theme::Palette;

use super::session::MusicSession;

pub enum MusicAction {
    GoHome,
}

pub fn show(
    ctx: &egui::Context,
    palette: Palette,
    session: &mut MusicSession,
) -> Option<MusicAction> {
    session.tick();

    let mut go_home = false;

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                let back = ui.add_sized(
                    egui::vec2(120.0, 50.0),
                    egui::Button::new(RichText::new("Back").size(24.0).color(palette.accent_text))
                        .fill(palette.accent),
                );

                if back.clicked() {
                    go_home = true;
                }

                ui.add_space(18.0);
                ui.label(
                    RichText::new(format!("Music: {} tracks", session.tracks_count()))
                        .size(28.0)
                        .color(palette.foreground),
                );
            });

            ui.add_space(14.0);

            Frame::new()
                .fill(palette.card)
                .stroke(Stroke::new(1.0, palette.border))
                .corner_radius(CornerRadius::same(20))
                .inner_margin(Margin::same(20))
                .show(ui, |ui| {
                    ui.set_min_height(360.0);
                    ui.vertical_centered(|ui| {
                        if let Some(texture) = session.current_cover(ctx) {
                            let side = ui.available_width().min(320.0);
                            ui.add(
                                egui::Image::new(texture)
                                    .fit_to_exact_size(egui::vec2(side, side))
                                    .corner_radius(CornerRadius::same(16)),
                            );
                        } else {
                            ui.allocate_ui_with_layout(
                                egui::vec2(320.0, 320.0),
                                egui::Layout::top_down(Align::Center),
                                |ui| {
                                    ui.vertical_centered(|ui| {
                                        ui.add_space(136.0);
                                        ui.label(
                                            RichText::new("No Cover Art")
                                                .size(28.0)
                                                .color(palette.muted),
                                        );
                                    });
                                },
                            );
                        }

                        ui.add_space(14.0);

                        if let Some(track) = session.current_track() {
                            ui.label(
                                RichText::new(&track.title)
                                    .size(34.0)
                                    .color(palette.foreground)
                                    .strong(),
                            );
                            ui.label(
                                RichText::new(format!("{} - {}", track.artist, track.album))
                                    .size(24.0)
                                    .color(palette.muted),
                            );
                        } else {
                            ui.label(
                                RichText::new("Press Shuffle to start")
                                    .size(30.0)
                                    .color(palette.muted),
                            );
                        }
                    });
                });

            ui.add_space(18.0);

            ui.horizontal_centered(|ui| {
                let btn_size = egui::vec2(180.0, 70.0);

                if ui
                    .add_sized(
                        btn_size,
                        egui::Button::new(
                            RichText::new("Shuffle")
                                .size(28.0)
                                .color(palette.accent_text),
                        )
                        .fill(palette.accent),
                    )
                    .clicked()
                {
                    session.shuffle_all();
                }

                if ui
                    .add_sized(
                        btn_size,
                        egui::Button::new(RichText::new("Prev").size(28.0)),
                    )
                    .clicked()
                {
                    session.previous();
                }

                let play_label = if session.is_playing() {
                    "Pause"
                } else {
                    "Play"
                };
                if ui
                    .add_sized(
                        btn_size,
                        egui::Button::new(RichText::new(play_label).size(28.0)),
                    )
                    .clicked()
                {
                    session.play_pause_toggle();
                }

                if ui
                    .add_sized(
                        btn_size,
                        egui::Button::new(RichText::new("Next").size(28.0)),
                    )
                    .clicked()
                {
                    session.next();
                }
            });

            if let Some(err) = session.last_error() {
                ui.add_space(10.0);
                ui.label(RichText::new(err).size(22.0).color(egui::Color32::RED));
            }
        });
    });

    if go_home {
        Some(MusicAction::GoHome)
    } else {
        None
    }
}
