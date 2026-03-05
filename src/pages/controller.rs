use std::{fs, path::Path};

use eframe::egui;
use egui::Color32;
use serde::{Deserialize, Serialize};

use crate::theme::{Palette, ThemeMode};

use super::{home, settings};

#[derive(Clone, Copy, PartialEq)]
enum AppView {
    Home,
    Settings,
}

const SETTINGS_PATH: &str = "./settings.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PersistedSettings {
    dark_mode: bool,
    accent_color: [u8; 4],
}

impl Default for PersistedSettings {
    fn default() -> Self {
        Self {
            dark_mode: true,
            accent_color: [94, 129, 255, 255],
        }
    }
}

impl PersistedSettings {
    fn load(path: &Path) -> Self {
        let Ok(content) = fs::read_to_string(path) else {
            return Self::default();
        };

        serde_json::from_str(&content).unwrap_or_default()
    }

    fn save(&self, path: &Path) {
        let Ok(content) = serde_json::to_string_pretty(self) else {
            return;
        };

        if let Err(err) = fs::write(path, content) {
            eprintln!("Failed to write {}: {err}", path.display());
        }
    }

    fn to_runtime(&self) -> (ThemeMode, Color32) {
        let mode = if self.dark_mode {
            ThemeMode::Dark
        } else {
            ThemeMode::Light
        };

        let [r, g, b, a] = self.accent_color;
        (mode, Color32::from_rgba_premultiplied(r, g, b, a))
    }

    fn from_runtime(theme_mode: ThemeMode, accent_color: Color32) -> Self {
        Self {
            dark_mode: theme_mode.is_dark(),
            accent_color: [
                accent_color.r(),
                accent_color.g(),
                accent_color.b(),
                accent_color.a(),
            ],
        }
    }
}

pub struct Controller {
    view: AppView,
    theme_mode: ThemeMode,
    accent_color: Color32,
    style_dirty: bool,
}

impl Controller {
    pub fn new() -> Self {
        let settings_path = Path::new(SETTINGS_PATH);
        let persisted = PersistedSettings::load(settings_path);
        if !settings_path.exists() {
            persisted.save(settings_path);
        }
        let (theme_mode, accent_color) = persisted.to_runtime();

        Self {
            view: AppView::Home,
            theme_mode,
            accent_color,
            style_dirty: true,
        }
    }

    fn save_settings(&self) {
        PersistedSettings::from_runtime(self.theme_mode, self.accent_color)
            .save(Path::new(SETTINGS_PATH));
    }

    pub fn palette(&self) -> Palette {
        self.theme_mode.palette().with_accent(self.accent_color)
    }

    pub fn style_needs_refresh(&self) -> bool {
        self.style_dirty
    }

    pub fn mark_style_applied(&mut self) {
        self.style_dirty = false;
    }

    pub fn update(&mut self, ctx: &egui::Context) {
        let palette = self.palette();

        match self.view {
            AppView::Home => {
                if let Some(home::HomeAction::OpenSettings) = home::show(ctx, palette) {
                    self.view = AppView::Settings;
                }
            }
            AppView::Settings => {
                let outcome =
                    settings::show(ctx, palette, &mut self.theme_mode, &mut self.accent_color);

                if outcome.style_changed {
                    self.style_dirty = true;
                    self.save_settings();
                }

                if outcome.go_home {
                    self.view = AppView::Home;
                }
            }
        }
    }
}
