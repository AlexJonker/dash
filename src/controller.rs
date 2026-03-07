use eframe::egui;

use crate::theme::Palette;

use super::{home, music, settings};

#[derive(Clone, Copy, PartialEq)]
enum AppView {
    Home,
    Music,
    Settings,
    AndroidAutoMenu,
}

pub struct Controller {
    view: AppView,
    settings_session: settings::SettingsSession,
    music_session: music::MusicSession,
}

impl Controller {
    pub fn new() -> Self {
        let settings_session = settings::SettingsSession::load();

        Self {
            view: AppView::Home,
            music_session: music::MusicSession::new(&settings_session.music_folder),
            settings_session,
        }
    }

    pub fn palette(&self) -> Palette {
        self.settings_session.palette()
    }

    pub fn update(&mut self, ctx: &egui::Context) {
        let palette = self.palette();

        match self.view {
            AppView::Home => {
                if let Some(action) = home::show(ctx, palette, self.settings_session.clock_format) {
                    match action {
                        home::HomeAction::OpenMusic => self.view = AppView::Music,
                        home::HomeAction::OpenSettings => self.view = AppView::Settings,
                        home::HomeAction::OpenAndroidAuto => self.view = AppView::AndroidAutoMenu,
                    }
                }
            }
            AppView::Music => {
                if let Some(music::MusicAction::GoHome) =
                    music::show(ctx, palette, &mut self.music_session)
                {
                    self.view = AppView::Home;
                }
            }
            AppView::Settings => {
                let old_music_folder = self.settings_session.music_folder.clone();
                let outcome = settings::show(
                    ctx,
                    palette,
                    &mut self.settings_session.theme_mode,
                    &mut self.settings_session.accent_color,
                    &mut self.settings_session.clock_format,
                    &mut self.settings_session.music_folder,
                );

                if outcome.settings_changed {
                    if self.settings_session.music_folder != old_music_folder {
                        self.music_session =
                            music::MusicSession::new(&self.settings_session.music_folder);
                    }

                    self.settings_session.save();
                }

                if outcome.go_home {
                    self.view = AppView::Home;
                }
            }
            AppView::AndroidAutoMenu => {
                if let Some(home::AndroidAutoMenuAction::GoHome) =
                    home::show_android_auto_menu(ctx, palette)
                {
                    self.view = AppView::Home;
                }
            }
        }
    }
}
