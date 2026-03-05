use eframe::egui;
use egui::{Align, CornerRadius, Frame, Layout, Margin, Stroke};

use crate::theme::{Palette, ThemeMode};

pub struct SettingsOutcome {
    pub go_home: bool,
    pub style_changed: bool,
}

pub fn show(
    ctx: &egui::Context,
    palette: Palette,
    theme_mode: &mut ThemeMode,
    accent_color: &mut egui::Color32,
) -> SettingsOutcome {
    let mut outcome = SettingsOutcome {
        go_home: false,
        style_changed: false,
    };

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.set_width(ui.available_width());

        ui.vertical_centered(|ui| {
            ui.add_space(8.0);
            ui.label(
                egui::RichText::new("Settings")
                    .size(28.0)
                    .color(palette.foreground),
            );
        });

        ui.add_space(20.0);

        Frame::new()
            .fill(palette.card)
            .stroke(Stroke::new(1.0, palette.border))
            .corner_radius(CornerRadius::same(18))
            .inner_margin(Margin::symmetric(16, 14))
            .show(ui, |ui| {
                let before = theme_mode.is_dark();
                let mut dark_mode = before;

                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(
                            egui::RichText::new("Dark mode")
                                .size(18.0)
                                .color(palette.foreground),
                        );
                        ui.label(
                            egui::RichText::new("Switch between dark and light themes")
                                .size(14.0)
                                .color(palette.muted),
                        );
                    });

                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        let label = if dark_mode { "On" } else { "Off" };
                        let response = ui.toggle_value(&mut dark_mode, label);
                        if response.changed() && dark_mode != before {
                            theme_mode.set_dark(dark_mode);
                            outcome.style_changed = true;
                        }
                    });
                });
            });

        ui.add_space(12.0);

        Frame::new()
            .fill(palette.card)
            .stroke(Stroke::new(1.0, palette.border))
            .corner_radius(CornerRadius::same(18))
            .inner_margin(Margin::symmetric(16, 14))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(
                            egui::RichText::new("Accent color")
                                .size(18.0)
                                .color(palette.foreground),
                        );
                        ui.label(
                            egui::RichText::new("Pick a color used for highlights and key actions")
                                .size(14.0)
                                .color(palette.muted),
                        );
                    });

                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if ui.color_edit_button_srgba(accent_color).changed() {
                            outcome.style_changed = true;
                        }
                    });
                });
            });

        ui.add_space(16.0);

        let back = ui.add(
            egui::Button::new(egui::RichText::new("Back to home").color(palette.accent_text))
                .min_size(egui::vec2(160.0, 38.0))
                .fill(palette.accent)
                .stroke(Stroke::new(1.0, palette.accent_hover)),
        );

        if back.clicked() {
            outcome.go_home = true;
        }
    });

    outcome
}
