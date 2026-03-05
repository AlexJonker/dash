use eframe::egui;
use egui::Color32;

use crate::theme::{Palette, ThemeMode};

use super::{home, settings};

#[derive(Clone, Copy, PartialEq)]
enum AppView {
    Home,
    Settings,
}

pub struct Controller {
    view: AppView,
    theme_mode: ThemeMode,
    accent_color: Color32,
    clock_type: u8,
    style_dirty: bool,
}

impl Controller {
    pub fn new() -> Self {
        let state = settings::load_state();

        Self {
            view: AppView::Home,
            theme_mode: state.theme_mode,
            accent_color: state.accent_color,
            clock_type: state.clock_type,
            style_dirty: true,
        }
    }

    fn save_settings(&self) {
        settings::save_state(settings::SettingsState {
            theme_mode: self.theme_mode,
            accent_color: self.accent_color,
            clock_type: self.clock_type,
        });
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
                if let Some(home::HomeAction::OpenSettings) =
                    home::show(ctx, palette, self.clock_type)
                {
                    self.view = AppView::Settings;
                }
            }
            AppView::Settings => {
                let outcome = settings::show(
                    ctx,
                    palette,
                    &mut self.theme_mode,
                    &mut self.accent_color,
                    &mut self.clock_type,
                );

                if outcome.style_changed {
                    self.style_dirty = true;
                }

                if outcome.settings_changed {
                    self.save_settings();
                }

                if outcome.go_home {
                    self.view = AppView::Home;
                }
            }
        }
    }
}
