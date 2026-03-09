use eframe::egui;
use egui::{Frame, RichText, ScrollArea, Vec2};

use super::super::session::MusicSession;
use crate::theme::Palette;

#[derive(Clone, PartialEq)]
enum Page {
    Artists,
    Artist(String),
    Album(String, String),
}

#[derive(Clone)]
pub struct BrowserState {
    page: Page,
}

impl Default for BrowserState {
    fn default() -> Self {
        Self {
            page: Page::Artists,
        }
    }
}

pub fn show_browser(
    ctx: &egui::Context,
    palette: Palette,
    session: &mut MusicSession,
    state: &mut BrowserState,
) {
    egui::CentralPanel::default()
        .frame(Frame::new().fill(palette.background))
        .show(ctx, |ui| {
            ui.set_width(ui.available_width());
            ui.add_space(8.0);

            match &state.page {
                Page::Artists => {
                    ui.label(
                        RichText::new("Artists")
                            .size(22.0)
                            .strong()
                            .color(palette.foreground),
                    );
                    ui.add_space(10.0);

                    let all_indices: Vec<usize> = session
                        .unique_artists()
                        .iter()
                        .flat_map(|artist| session.tracks_for_artist(artist))
                        .map(|(i, _, _)| i)
                        .collect();

                    if nav_button(ui, palette, "▶ Play All") && !all_indices.is_empty() {
                        session.play_all_from_list(&all_indices);
                    }
                    if let Some(a) = show_list(
                        ui,
                        palette,
                        &session.unique_artists(),
                        session.current_track().map(|t| &t.artist[..]),
                    ) {
                        state.page = Page::Artist(a);
                    }
                }
                Page::Artist(artist) => {
                    if nav_button(ui, palette, &format!("← Artists")) {
                        state.page = Page::Artists;
                        return;
                    }
                    ui.add_space(4.0);
                    ui.label(
                        RichText::new(artist)
                            .size(22.0)
                            .strong()
                            .color(palette.foreground),
                    );
                    ui.add_space(10.0);

                    let tracks = session.tracks_for_artist(artist);
                    let indices: Vec<_> = tracks.iter().map(|(i, _, _)| *i).collect();
                    if nav_button(ui, palette, "▶ Play All") && !indices.is_empty() {
                        session.play_all_from_list(&indices);
                    }
                    ui.add_space(10.0);

                    if let Some(album) = show_list(
                        ui,
                        palette,
                        &session.albums_for_artist(artist),
                        session
                            .current_track()
                            .filter(|t| &t.artist == artist)
                            .map(|t| &t.album[..]),
                    ) {
                        state.page = Page::Album(artist.clone(), album);
                    }
                }
                Page::Album(artist, album) => {
                    if nav_button(ui, palette, &format!("← {}", artist)) {
                        state.page = Page::Artist(artist.clone());
                        return;
                    }
                    ui.add_space(4.0);
                    ui.label(
                        RichText::new(album)
                            .size(22.0)
                            .strong()
                            .color(palette.foreground),
                    );
                    ui.label(RichText::new(artist).size(15.0).color(palette.muted));
                    ui.add_space(10.0);

                    let tracks = session.tracks_for_album(artist, album);
                    let indices: Vec<_> = tracks.iter().map(|(i, _, _)| *i).collect();
                    if nav_button(ui, palette, "▶ Play All") && !indices.is_empty() {
                        session.play_all_from_list(&indices);
                    }
                    ui.add_space(10.0);
                    show_tracks(ui, palette, session, &tracks, &indices);
                }
            }
        });
}

fn nav_button(ui: &mut egui::Ui, palette: Palette, text: &str) -> bool {
    let btn = egui::Button::new(RichText::new(text).strong().color(palette.accent_text))
        .fill(palette.accent)
        .min_size(Vec2::new(140.0, 36.0))
        .corner_radius(8.0);
    ui.add(btn).clicked()
}

fn show_list(
    ui: &mut egui::Ui,
    palette: Palette,
    items: &[String],
    active_item: Option<&str>,
) -> Option<String> {
    if items.is_empty() {
        empty(ui, palette, "No items found");
        return None;
    }

    let mut chosen = None;
    ScrollArea::vertical().show(ui, |ui| {
        for item in items {
            if list_row(ui, palette, item, active_item == Some(item)).clicked() {
                chosen = Some(item.clone());
            }
        }
    });
    chosen
}

fn show_tracks(
    ui: &mut egui::Ui,
    palette: Palette,
    session: &mut MusicSession,
    tracks: &[(usize, String, String)],
    queue: &[usize],
) {
    if tracks.is_empty() {
        empty(ui, palette, "No tracks found");
        return;
    }

    let current = session.current_track_index_pub();
    let mut to_play = None;
    ScrollArea::vertical().show(ui, |ui| {
        for (pos, (idx, _, _)) in tracks.iter().enumerate() {
            let title = session
                .track_title(*idx)
                .unwrap_or_else(|| "Unknown".into());
            if list_row(ui, palette, &title, current == Some(*idx)).clicked() {
                to_play = Some(pos);
            }
        }
    });

    if let Some(pos) = to_play {
        session.play_from_list(queue, pos);
    }
}

fn list_row(ui: &mut egui::Ui, palette: Palette, text: &str, active: bool) -> egui::Response {
    let height = 44.0;
    let desired = Vec2::new(ui.available_width(), height);
    let (rect, resp) = ui.allocate_exact_size(desired, egui::Sense::click());

    if ui.is_rect_visible(rect) {
        let bg = if active {
            palette.accent
        } else if resp.hovered() {
            palette.card_hover
        } else {
            palette.card
        };
        ui.painter().rect_filled(rect, 8.0, bg);
        ui.painter().text(
            rect.left_top() + Vec2::new(8.0, height / 2.0 - 8.0),
            egui::Align2::LEFT_CENTER,
            text,
            egui::FontId::proportional(16.0),
            if active {
                palette.foreground
            } else {
                palette.foreground
            },
        );
    }

    resp
}

fn empty(ui: &mut egui::Ui, palette: Palette, msg: &str) {
    ui.vertical_centered(|ui| {
        ui.add_space(40.0);
        ui.label(RichText::new(msg).size(18.0).color(palette.muted));
    });
}
