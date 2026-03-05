use eframe::egui;
use egui::{Align, CornerRadius, Frame, Layout, Margin, Stroke};

use crate::theme::{Palette, ThemeMode};

pub struct SettingsOutcome {
    pub go_home: bool,
    pub theme_changed: bool,
}

pub fn show(ctx: &egui::Context, palette: Palette, theme_mode: &mut ThemeMode) -> SettingsOutcome {
    let mut outcome = SettingsOutcome {
        go_home: false,
        theme_changed: false,
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
            ui.add_space(6.0);
            ui.label(
                egui::RichText::new("Tune the dash to your taste")
                    .size(16.0)
                    .color(palette.muted),
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
                            outcome.theme_changed = true;
                        }
                    });
                });
            });

        ui.add_space(16.0);

        let back = ui.add(
            egui::Button::new("Back to home")
                .min_size(egui::vec2(160.0, 38.0))
                .fill(palette.card_hover)
                .stroke(Stroke::new(1.0, palette.border)),
        );

        if back.clicked() {
            outcome.go_home = true;
        }
    });

    outcome
}
