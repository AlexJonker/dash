use chrono::Local;
use eframe::egui;
use egui::{Align2, CornerRadius, Frame, Margin, Stroke, Vec2};

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
            music_session: music::MusicSession::new(
                &settings_session.music_folder,
                settings_session.music_volume,
            ),
            settings_session,
        }
    }

    pub fn palette(&self) -> Palette {
        self.settings_session.palette()
    }

    pub fn update(&mut self, ctx: &egui::Context) {
        let palette = self.palette();

        self.show_top_bar(ctx, palette);

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
                if let Some(new_volume) = music::show(ctx, palette, &mut self.music_session) {
                    self.settings_session.music_volume = new_volume;
                    self.settings_session.save();
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
                        self.music_session = music::MusicSession::new(
                            &self.settings_session.music_folder,
                            self.settings_session.music_volume,
                        );
                    }

                    self.settings_session.save();
                }
            }
            AppView::AndroidAutoMenu => home::show_android_auto_menu(ctx, palette),
        }
    }

    fn show_top_bar(&mut self, ctx: &egui::Context, palette: Palette) {
        let now = Local::now();
        let time_text = if self.settings_session.clock_format.is_12h() {
            now.format("%-I:%M %p").to_string()
        } else {
            now.format("%H:%M").to_string()
        };

        let is_home = self.view == AppView::Home;

        let bar_surface = {
            let bg = palette.background;
            let tint = palette.card;
            egui::Color32::from_rgb(
                ((bg.r() as u16 * 3 + tint.r() as u16) / 4) as u8,
                ((bg.g() as u16 * 3 + tint.g() as u16) / 4) as u8,
                ((bg.b() as u16 * 3 + tint.b() as u16) / 4) as u8,
            )
        };

        egui::TopBottomPanel::top("top_bar")
            .frame(
                Frame::default()
                    .fill(bar_surface)
                    .inner_margin(Margin::symmetric(16, 10))
                    .stroke(Stroke::NONE),
            )
            .show(ctx, |ui| {
                let bar_height = 56.0;
                let (bar_rect, _) = ui.allocate_exact_size(
                    Vec2::new(ui.available_width(), bar_height),
                    egui::Sense::hover(),
                );

                let painter = ui.painter_at(bar_rect);

                if !is_home {
                    let btn_w = 112.0;
                    let btn_h = 40.0;
                    let btn_rect = egui::Rect::from_center_size(
                        bar_rect.left_center() + egui::vec2(btn_w / 2.0 + 4.0, 0.0),
                        Vec2::new(btn_w, btn_h),
                    );

                    let home_response = ui.interact(
                        btn_rect,
                        ui.make_persistent_id("top_home"),
                        egui::Sense::click(),
                    );

                    let btn_fill = if home_response.hovered() {
                        palette.card_hover
                    } else {
                        palette.card
                    };

                    let pill = CornerRadius::same((btn_h / 2.0) as u8);
                    painter.rect_filled(btn_rect, pill, btn_fill);

                    let arrow_x = btn_rect.left() + 20.0;
                    let arrow_y = btn_rect.center().y;
                    let chevron_stroke = Stroke::new(2.0, palette.foreground);
                    painter.line_segment(
                        [
                            egui::pos2(arrow_x + 5.0, arrow_y - 6.0),
                            egui::pos2(arrow_x - 1.0, arrow_y),
                        ],
                        chevron_stroke,
                    );
                    painter.line_segment(
                        [
                            egui::pos2(arrow_x - 1.0, arrow_y),
                            egui::pos2(arrow_x + 5.0, arrow_y + 6.0),
                        ],
                        chevron_stroke,
                    );

                    painter.text(
                        egui::pos2(arrow_x + 18.0, btn_rect.center().y),
                        Align2::LEFT_CENTER,
                        "Home",
                        egui::FontId::proportional(16.0),
                        palette.foreground,
                    );

                    if home_response.clicked() {
                        self.view = AppView::Home;
                    }
                }

                let time_font = egui::FontId::proportional(24.0);
                let time_galley = painter.layout_no_wrap(
                    time_text.clone(),
                    time_font.clone(),
                    palette.foreground,
                );

                let chip_pad_x = 20.0;
                let chip_h = 40.0;
                let chip_w = time_galley.size().x + chip_pad_x * 2.0;
                let chip_rect =
                    egui::Rect::from_center_size(bar_rect.center(), Vec2::new(chip_w, chip_h));

                let chip_fill = egui::Color32::from_rgba_premultiplied(
                    palette.card.r(),
                    palette.card.g(),
                    palette.card.b(),
                    160,
                );
                painter.rect_filled(
                    chip_rect,
                    CornerRadius::same((chip_h / 2.0) as u8),
                    chip_fill,
                );

                painter.text(
                    chip_rect.center(),
                    Align2::CENTER_CENTER,
                    time_text,
                    egui::FontId::proportional(24.0),
                    palette.foreground,
                );
            });
    }
}
