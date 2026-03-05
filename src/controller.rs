use eframe::egui;

use crate::theme::Palette;

use super::{home, settings};

#[derive(Clone, Copy, PartialEq)]
enum AppView {
    Home,
    Settings,
}

pub struct Controller {
    view: AppView,
    settings_session: settings::SettingsSession,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            view: AppView::Home,
            settings_session: settings::SettingsSession::load(),
        }
    }

    pub fn palette(&self) -> Palette {
        self.settings_session.palette()
    }

    pub fn update(&mut self, ctx: &egui::Context) {
        let palette = self.palette();

        match self.view {
            AppView::Home => {
                if let Some(home::HomeAction::OpenSettings) =
                    home::show(ctx, palette, self.settings_session.clock_format)
                {
                    self.view = AppView::Settings;
                }
            }
            AppView::Settings => {
                let outcome = settings::show(
                    ctx,
                    palette,
                    &mut self.settings_session.theme_mode,
                    &mut self.settings_session.accent_color,
                    &mut self.settings_session.clock_format,
                );

                if outcome.settings_changed {
                    self.settings_session.save();
                }

                if outcome.go_home {
                    self.view = AppView::Home;
                }
            }
        }
    }
}
