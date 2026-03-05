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
    style_dirty: bool,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            view: AppView::Home,
            theme_mode: ThemeMode::Dark,
            accent_color: Color32::from_rgb(94, 129, 255),
            style_dirty: true,
        }
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
                }

                if outcome.go_home {
                    self.view = AppView::Home;
                }
            }
        }
    }
}
