use eframe::egui;
use egui::{Align, CornerRadius, Frame, Margin, RichText, Stroke};

use super::session::MusicSession;
use crate::theme::Palette;

pub enum MusicAction {
    GoHome,
}

pub fn show(
    ctx: &egui::Context,
    palette: Palette,
    session: &mut MusicSession,
) -> Option<MusicAction> {
    session.tick();
    ctx.request_repaint();
    let mut go_home = false;

    egui::CentralPanel::default().show(ctx, |ui| {
        // Top bar
        ui.horizontal(|ui| {
            if ui
                .add_sized(
                    [110.0, 50.0],
                    egui::Button::new(RichText::new("Back").size(24.0).color(palette.accent_text))
                        .fill(palette.accent),
                )
                .clicked()
            {
                go_home = true;
            }
            ui.add_space(16.0);
            ui.label(
                RichText::new(format!("Music • {} tracks", session.tracks_count()))
                    .size(28.0)
                    .color(palette.foreground),
            );
        });

        ui.add_space(12.0);

        Frame::new()
            .fill(palette.card)
            .stroke(Stroke::new(1.0, palette.border))
            .corner_radius(CornerRadius::same(20))
            .inner_margin(Margin::same(20))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    // Cover art
                    let cover_size = egui::vec2(260.0, 260.0);
                    if let Some(tex) = session.current_cover(ctx) {
                        ui.add(
                            egui::Image::new(tex)
                                .fit_to_exact_size(cover_size)
                                .corner_radius(CornerRadius::same(16)),
                        );
                    } else {
                        ui.allocate_ui_with_layout(
                            cover_size,
                            egui::Layout::top_down(Align::Center),
                            |ui| {
                                ui.add_space(110.0);
                                ui.label(RichText::new("No Cover").size(26.0).color(palette.muted));
                            },
                        );
                    }

                    ui.add_space(24.0);

                    ui.vertical(|ui| {
                        let Some(track) = session.current_track() else {
                            ui.add_space(120.0);
                            ui.label(
                                RichText::new("Press Shuffle to start")
                                    .size(32.0)
                                    .color(palette.muted),
                            );
                            return;
                        };

                        ui.label(
                            RichText::new(&track.title)
                                .size(36.0)
                                .strong()
                                .color(palette.foreground),
                        );
                        ui.label(
                            RichText::new(format!("{} • {}", track.artist, track.album))
                                .size(24.0)
                                .color(palette.muted),
                        );
                        ui.add_space(18.0);

                        // Progress slider
                        if let Some(total) = session.current_duration_secs() {
                            let mut pos = session.current_position_secs().min(total);
                            let width = ui.available_width();

                            let resp = ui
                                .scope(|ui| {
                                    let s = ui.style_mut();
                                    s.spacing.slider_width = width;
                                    s.spacing.interact_size.y = 50.0;
                                    let v = &mut s.visuals.widgets;
                                    v.inactive.bg_stroke = Stroke::new(10.0, palette.card_hover);
                                    v.hovered.bg_stroke = Stroke::new(10.0, palette.card_hover);
                                    v.active.bg_fill = palette.accent;
                                    v.hovered.bg_fill = palette.accent;
                                    v.inactive.bg_fill = palette.accent;
                                    v.inactive.fg_stroke.color = palette.accent_text;
                                    v.hovered.fg_stroke.color = palette.accent_text;
                                    v.active.fg_stroke.color = palette.accent_text;
                                    ui.add_sized(
                                        [ui.available_width(), 50.0],
                                        egui::Slider::new(&mut pos, 0.0..=total).show_value(false),
                                    )
                                })
                                .inner;

                            if resp.dragged() || resp.drag_stopped() {
                                session.seek_to_secs(pos);
                            }

                            ui.horizontal(|ui| {
                                ui.label(
                                    RichText::new(format_time(pos))
                                        .size(18.0)
                                        .color(palette.muted),
                                );
                                ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                                    ui.label(
                                        RichText::new(format_time(total))
                                            .size(18.0)
                                            .color(palette.muted),
                                    );
                                });
                            });
                        }

                        ui.add_space(26.0);

                        // Controls
                        ui.horizontal(|ui| {
                            let btn = [140.0, 64.0];
                            if ui
                                .add_sized(
                                    btn,
                                    egui::Button::new(
                                        RichText::new("Shuffle")
                                            .size(24.0)
                                            .color(palette.accent_text),
                                    )
                                    .fill(palette.accent),
                                )
                                .clicked()
                            {
                                session.shuffle_all();
                            }
                            if ui
                                .add_sized(btn, egui::Button::new(RichText::new("Prev").size(24.0)))
                                .clicked()
                            {
                                session.previous();
                            }
                            if ui
                                .add_sized(
                                    btn,
                                    egui::Button::new(
                                        RichText::new(if session.is_playing() {
                                            "Pause"
                                        } else {
                                            "Play"
                                        })
                                        .size(26.0),
                                    ),
                                )
                                .clicked()
                            {
                                session.play_pause_toggle();
                            }
                            if ui
                                .add_sized(btn, egui::Button::new(RichText::new("Next").size(24.0)))
                                .clicked()
                            {
                                session.next();
                            }
                        });
                    });
                });
            });

        if let Some(err) = session.last_error() {
            ui.add_space(8.0);
            ui.label(RichText::new(err).size(20.0).color(egui::Color32::RED));
        }
    });

    go_home.then_some(MusicAction::GoHome)
}

fn format_time(secs: f32) -> String {
    let s = secs.max(0.0).round() as u64;
    format!("{:02}:{:02}", s / 60, s % 60)
}
