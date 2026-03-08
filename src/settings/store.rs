use std::{fs, path::Path};

use egui::Color32;
use serde::{Deserialize, Serialize};

use crate::theme::{ClockFormat, ThemeMode};

const SETTINGS_PATH: &str = "./settings.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PersistedSettings {
    dark_mode: bool,
    accent_color: [u8; 4],
    #[serde(default)]
    clock_type: u8,
    #[serde(default)]
    music_folder: String,
    #[serde(default)]
    music_volume: f32,
}

// Default settings
impl Default for PersistedSettings {
    fn default() -> Self {
        Self {
            dark_mode: true,
            accent_color: [94, 129, 255, 255],
            clock_type: 24,
            music_folder: "/storage/music".to_string(),
            music_volume: 0.8,
        }
    }
}

#[derive(Clone)]
pub struct SettingsState {
    pub theme_mode: ThemeMode,
    pub accent_color: Color32,
    pub clock_format: ClockFormat,
    pub music_folder: String,
    pub music_volume: f32,
}

impl PersistedSettings {
    fn load(path: &Path) -> Self {
        let Ok(content) = fs::read_to_string(path) else {
            return Self::default();
        };

        let settings: Self = serde_json::from_str(&content).unwrap_or_default();

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
            clock_format: ClockFormat::from_u8(self.clock_type),
            music_folder: self.music_folder.clone(),
            music_volume: self.music_volume.clamp(0.0, 1.0),
        }
    }

    fn from_state(state: SettingsState) -> Self {
        Self {
            dark_mode: state.theme_mode.is_dark(),
            accent_color: [
                state.accent_color.r(),
                state.accent_color.g(),
                state.accent_color.b(),
                state.accent_color.a(),
            ],
            clock_type: state.clock_format.as_u8(),
            music_folder: state.music_folder,
            music_volume: state.music_volume.clamp(0.0, 1.0),
        }
    }
}

pub fn load_state() -> SettingsState {
    let settings_path = Path::new(SETTINGS_PATH);
    let persisted = PersistedSettings::load(settings_path);

    if !settings_path.exists() || persisted.music_folder.trim().is_empty() {
        persisted.save(settings_path);
    }

    persisted.to_state()
}

pub fn save_state(state: SettingsState) {
    PersistedSettings::from_state(state).save(Path::new(SETTINGS_PATH));
}
