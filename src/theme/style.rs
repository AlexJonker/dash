use eframe::egui;
use egui::{CornerRadius, Margin};

use super::Palette;

pub fn apply_style(ctx: &egui::Context, palette: Palette) {
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
