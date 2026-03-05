use eframe::egui;

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
    style_dirty: bool,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            view: AppView::Home,
            theme_mode: ThemeMode::Dark,
            style_dirty: true,
        }
    }

    pub fn palette(&self) -> Palette {
        self.theme_mode.palette()
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
                let outcome = settings::show(ctx, palette, &mut self.theme_mode);

                if outcome.theme_changed {
                    self.style_dirty = true;
                }

                if outcome.go_home {
                    self.view = AppView::Home;
                }
            }
        }
    }
}
