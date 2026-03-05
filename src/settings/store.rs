use std::{fs, path::Path};

use egui::Color32;
use serde::{Deserialize, Serialize};

use crate::theme::ThemeMode;

const SETTINGS_PATH: &str = "./settings.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PersistedSettings {
    dark_mode: bool,
    accent_color: [u8; 4],
    #[serde(default)]
    clock_type: u8,
}

impl Default for PersistedSettings {
    fn default() -> Self {
        Self {
            dark_mode: true,
            accent_color: [94, 129, 255, 255],
            clock_type: 24,
        }
    }
}

#[derive(Clone, Copy)]
pub struct SettingsState {
    pub theme_mode: ThemeMode,
    pub accent_color: Color32,
    pub clock_type: u8,
}

impl PersistedSettings {
    fn load(path: &Path) -> Self {
        let Ok(content) = fs::read_to_string(path) else {
            return Self::default();
        };

        let mut settings: Self = serde_json::from_str(&content).unwrap_or_default();

        if settings.clock_type != 12 && settings.clock_type != 24 {
            settings.clock_type = 24;
        }

        settings
    }

    fn save(&self, path: &Path) {
        let Ok(content) = serde_json::to_string_pretty(self) else {
            return;
        };

        if let Err(err) = fs::write(path, content) {
            eprintln!("Failed to write {}: {err}", path.display());
        }
    }

    fn to_state(&self) -> SettingsState {
        let theme_mode = if self.dark_mode {
            ThemeMode::Dark
        } else {
            ThemeMode::Light
        };

        let [r, g, b, a] = self.accent_color;

        SettingsState {
            theme_mode,
            accent_color: Color32::from_rgba_premultiplied(r, g, b, a),
            clock_type: self.clock_type,
        }
    }

    fn from_state(state: SettingsState) -> Self {
        let normalized_clock_type = if state.clock_type == 12 { 12 } else { 24 };

        Self {
            dark_mode: state.theme_mode.is_dark(),
            accent_color: [
                state.accent_color.r(),
                state.accent_color.g(),
                state.accent_color.b(),
                state.accent_color.a(),
            ],
            clock_type: normalized_clock_type,
        }
    }
}

pub fn load_state() -> SettingsState {
    let settings_path = Path::new(SETTINGS_PATH);
    let persisted = PersistedSettings::load(settings_path);

    if !settings_path.exists() {
        persisted.save(settings_path);
    }

    persisted.to_state()
}

pub fn save_state(state: SettingsState) {
    PersistedSettings::from_state(state).save(Path::new(SETTINGS_PATH));
}
