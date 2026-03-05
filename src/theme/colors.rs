use egui::Color32;

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
