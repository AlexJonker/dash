use eframe::egui;
use egui::{Color32, CornerRadius, Margin};

#[derive(Clone, Copy, PartialEq)]
pub struct Palette {
    pub background: Color32,
    pub foreground: Color32,
    pub card: Color32,
    pub card_hover: Color32,
    pub accent: Color32,
    pub accent_hover: Color32,
    pub accent_text: Color32,
    pub muted: Color32,
    pub border: Color32,
}

#[derive(Clone, Copy, PartialEq)]
pub enum ThemeMode {
    Dark,
    Light,
}

impl ThemeMode {
    pub fn palette(self) -> Palette {
        match self {
            Self::Dark => Palette::dark(),
            Self::Light => Palette::light(),
        }
    }

    pub fn is_dark(self) -> bool {
        matches!(self, Self::Dark)
    }

    pub fn set_dark(&mut self, is_dark: bool) {
        *self = if is_dark { Self::Dark } else { Self::Light };
    }
}

impl Palette {
    pub fn dark() -> Self {
        let accent = Color32::from_rgb(94, 129, 255);
        Self {
            background: Color32::from_rgb(18, 18, 22),
            foreground: Color32::from_rgb(238, 238, 244),
            card: Color32::from_rgb(28, 28, 34),
            card_hover: Color32::from_rgb(36, 36, 44),
            accent,
            accent_hover: accent.gamma_multiply(1.15),
            accent_text: Color32::WHITE,
            muted: Color32::from_rgb(160, 160, 176),
            border: Color32::from_rgba_premultiplied(90, 90, 104, 120),
        }
    }

    pub fn light() -> Self {
        let accent = Color32::from_rgb(42, 106, 255);
        Self {
            background: Color32::from_rgb(242, 242, 245),
            foreground: Color32::from_rgb(28, 28, 34),
            card: Color32::from_rgb(255, 255, 255),
            card_hover: Color32::from_rgb(245, 245, 245),
            accent,
            accent_hover: accent.gamma_multiply(0.9),
            accent_text: Color32::WHITE,
            muted: Color32::from_rgb(120, 120, 136),
            border: Color32::from_rgba_premultiplied(90, 90, 104, 120),
        }
    }

    pub fn with_accent(mut self, accent: Color32) -> Self {
        self.accent = accent;
        self.accent_hover = accent.gamma_multiply(1.2);

        let luminance = 0.2126 * f32::from(accent.r())
            + 0.7152 * f32::from(accent.g())
            + 0.0722 * f32::from(accent.b());
        self.accent_text = if luminance > 150.0 {
            Color32::from_rgb(18, 18, 22)
        } else {
            Color32::WHITE
        };

        self
    }
}

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
