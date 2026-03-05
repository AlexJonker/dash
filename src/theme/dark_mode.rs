use crate::theme::Palette;

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
