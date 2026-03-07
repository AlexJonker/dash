use egui::Color32;

use crate::theme::{ClockFormat, Palette, ThemeMode};

use super::{SettingsState, load_state, save_state};

pub struct SettingsSession {
    pub theme_mode: ThemeMode,
    pub accent_color: Color32,
    pub clock_format: ClockFormat,
    pub music_folder: String,
}

impl SettingsSession {
    pub fn load() -> Self {
        let state = load_state();

        Self {
            theme_mode: state.theme_mode,
            accent_color: state.accent_color,
            clock_format: state.clock_format,
            music_folder: state.music_folder,
        }
    }

    pub fn palette(&self) -> Palette {
        self.theme_mode.palette().with_accent(self.accent_color)
    }

    pub fn save(&self) {
        save_state(SettingsState {
            theme_mode: self.theme_mode,
            accent_color: self.accent_color,
            clock_format: self.clock_format,
            music_folder: self.music_folder.clone(),
        });
    }
}
