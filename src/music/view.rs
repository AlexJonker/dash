use eframe::egui;
use egui::{
    Align, CornerRadius, Direction, Frame, ImageSource, Pos2, Rect, RichText, UiBuilder, Vec2,
};

use super::session::MusicSession;
use crate::theme::Palette;

#[derive(Clone, Copy)]
enum ControlIcon {
    Previous,
    Play,
    Pause,
    Next,
    MusicNote,
    Shuffle,
}

fn icon(icon: ControlIcon) -> ImageSource<'static> {
    match icon {
        ControlIcon::Previous => egui::include_image!("../../assets/icons/previous.svg"),
        ControlIcon::Play => egui::include_image!("../../assets/icons/play.svg"),
        ControlIcon::Pause => egui::include_image!("../../assets/icons/pause.svg"),
        ControlIcon::Next => egui::include_image!("../../assets/icons/next.svg"),
        ControlIcon::MusicNote => egui::include_image!("../../assets/icons/music.svg"),
        ControlIcon::Shuffle => egui::include_image!("../../assets/icons/shuffle.svg"),
    }
}

fn centered_child(ui: &mut egui::Ui, rect: Rect) -> egui::Ui {
    ui.new_child(
        UiBuilder::new()
            .max_rect(rect)
            .layout(egui::Layout::centered_and_justified(Direction::LeftToRight)),
    )
}

pub fn show(ctx: &egui::Context, palette: Palette, session: &mut MusicSession) -> Option<f32> {
    session.tick();
    ctx.request_repaint_after(std::time::Duration::from_millis(16));

    let mut volume_changed = None;

    egui::CentralPanel::default()
        .frame(Frame::new().fill(palette.background))
        .show(ctx, |ui| {
            let size = ui.available_size();
            let cx = size.x / 2.0;

            // Sizes
            let cover = (size.y * 0.38).min(260.0);
            let play = 150.0;
            let btn = 100.0;

            let gaps = (16.0, 14.0, 16.0);
            let shuffle_gap = 16.0;
            let meta_h = 96.0;
            let seek_h = 64.0;

            let block_h =
                cover + gaps.0 + meta_h + gaps.1 + seek_h + gaps.2 + play + shuffle_gap + btn;
            let cover_top = ((size.y - block_h) / 2.0).max(8.0);

            // Y Positions
            let meta_top = cover_top + cover + gaps.0;
            let seek_top = meta_top + meta_h + gaps.1;
            let ctrl_top = seek_top + seek_h + gaps.2;

            // Rects
            let cover_rect =
                Rect::from_center_size(Pos2::new(cx, cover_top + cover / 2.0), Vec2::splat(cover));

            let title_rect =
                Rect::from_center_size(Pos2::new(cx, meta_top + 18.0), Vec2::new(440.0, 36.0));
            let artist_rect =
                Rect::from_center_size(Pos2::new(cx, meta_top + 52.0), Vec2::new(440.0, 28.0));
            let album_rect =
                Rect::from_center_size(Pos2::new(cx, meta_top + 78.0), Vec2::new(440.0, 22.0));

            let bar_w = (size.x - 60.0).min(520.0);
            let seek_rect =
                Rect::from_center_size(Pos2::new(cx, seek_top + 22.0), Vec2::new(bar_w, 44.0));
            let time_rect =
                Rect::from_center_size(Pos2::new(cx, seek_top + 54.0), Vec2::new(bar_w, 20.0));

            let row_w = btn * 2.0 + play + 40.0;
            let ctrl_left = cx - row_w / 2.0;

            let prev_rect = Rect::from_min_size(
                Pos2::new(ctrl_left, ctrl_top + (play - btn) / 2.0),
                Vec2::splat(btn),
            );
            let play_rect = Rect::from_min_size(
                Pos2::new(ctrl_left + btn + 20.0, ctrl_top),
                Vec2::splat(play),
            );
            let next_rect = Rect::from_min_size(
                Pos2::new(
                    ctrl_left + btn + 20.0 + play + 20.0,
                    ctrl_top + (play - btn) / 2.0,
                ),
                Vec2::splat(btn),
            );
            let shuffle_rect = Rect::from_center_size(
                Pos2::new(cx, ctrl_top + play + shuffle_gap + btn / 2.0),
                Vec2::splat(btn),
            );

            // Album cover
            let mut cover_ui = centered_child(ui, cover_rect);
            if let Some(tex) = session.current_cover(ctx) {
                cover_ui.add(
                    egui::Image::new(tex)
                        .fit_to_exact_size(Vec2::splat(cover))
                        .corner_radius(CornerRadius::same(16)),
                );
            } else {
                cover_ui
                    .painter()
                    .rect_filled(cover_rect, 16.0, palette.card);
                centered_child(ui, cover_rect).add(
                    egui::Image::new(icon(ControlIcon::MusicNote))
                        .fit_to_exact_size(Vec2::splat(72.0))
                        .tint(palette.muted),
                );
            }

            // Title, artist, album
            if let Some(track) = session.current_track() {
                centered_child(ui, title_rect).label(
                    RichText::new(&track.title)
                        .size(28.0)
                        .strong()
                        .color(palette.foreground),
                );

                centered_child(ui, artist_rect)
                    .label(RichText::new(&track.artist).size(20.0).color(palette.muted));

                centered_child(ui, album_rect)
                    .label(RichText::new(&track.album).size(16.0).color(palette.muted));
            } else {
                centered_child(ui, title_rect).label(
                    RichText::new("Press Play to start")
                        .size(24.0)
                        .color(palette.muted),
                );
            }

            // Progress bar
            if let Some(total) = session.current_duration_secs() {
                let mut pos = session.current_position_secs().min(total);

                let resp = ui.allocate_rect(seek_rect, egui::Sense::drag());

                if resp.dragged() || resp.drag_stopped() {
                    let x = resp
                        .interact_pointer_pos()
                        .map(|p| p.x)
                        .unwrap_or(seek_rect.left());
                    let t = ((x - seek_rect.left()) / seek_rect.width()).clamp(0.0, 1.0);
                    pos = t * total;
                    session.seek_to_secs(pos);
                }

                let painter = ui.painter();
                let track = Rect::from_center_size(seek_rect.center(), Vec2::new(bar_w, 8.0));

                painter.rect_filled(track, 4.0, palette.card_hover);

                let fill = (pos / total * bar_w).clamp(0.0, bar_w);
                painter.rect_filled(
                    Rect::from_min_size(track.min, Vec2::new(fill, 8.0)),
                    4.0,
                    palette.accent,
                );
                painter.circle_filled(
                    Pos2::new(track.left() + fill, track.center().y),
                    10.0,
                    palette.accent,
                );

                let mut t_ui = ui.new_child(
                    UiBuilder::new()
                        .max_rect(time_rect)
                        .layout(egui::Layout::left_to_right(Align::Center)),
                );
                t_ui.label(
                    RichText::new(format_time(pos))
                        .size(14.0)
                        .color(palette.muted),
                );

                t_ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                    ui.label(
                        RichText::new(format_time(total))
                            .size(14.0)
                            .color(palette.muted),
                    );
                });
            }

            // Play buttons
            icon_btn_abs(
                ui,
                palette,
                icon(ControlIcon::Previous),
                prev_rect,
                false,
                || session.previous(),
            );

            icon_btn_abs(
                ui,
                palette,
                icon(if session.is_playing() {
                    ControlIcon::Pause
                } else {
                    ControlIcon::Play
                }),
                play_rect,
                true,
                || session.play_pause_toggle(),
            );

            icon_btn_abs(
                ui,
                palette,
                icon(ControlIcon::Next),
                next_rect,
                false,
                || session.next(),
            );

            icon_btn_abs(
                ui,
                palette,
                icon(ControlIcon::Shuffle),
                shuffle_rect,
                session.is_shuffle_enabled(),
                || session.shuffle_toggle(),
            );

            // Volume slider
            let volume = session.get_volume();
            let bar_h = (size.y * 0.28).clamp(120.0, 220.0);
            let bar_rect =
                Rect::from_center_size(Pos2::new(32.0, size.y / 2.0), Vec2::new(10.0, bar_h));

            let resp = ui.allocate_rect(bar_rect.expand(10.0), egui::Sense::click_and_drag());

            if resp.clicked() || resp.dragged() {
                let y = resp
                    .interact_pointer_pos()
                    .map(|p| p.y)
                    .unwrap_or(bar_rect.bottom());
                let t = 1.0 - ((y - bar_rect.top()) / bar_rect.height()).clamp(0.0, 1.0);
                session.set_volume(t);
                volume_changed = Some(session.get_volume());
            }

            let painter = ui.painter();
            painter.rect_filled(bar_rect, 4.0, palette.card_hover);

            let fill = (volume * bar_h).clamp(0.0, bar_h);
            painter.rect_filled(
                Rect::from_min_size(
                    Pos2::new(bar_rect.left(), bar_rect.bottom() - fill),
                    Vec2::new(bar_rect.width(), fill),
                ),
                4.0,
                palette.accent,
            );

            painter.circle_filled(
                Pos2::new(bar_rect.center().x, bar_rect.bottom() - fill),
                10.0,
                palette.accent,
            );

            // Volume percentage text
            let vol_pct = format!("{}%", (volume * 100.0).round() as u32);
            let label_rect = Rect::from_center_size(
                Pos2::new(bar_rect.center().x, bar_rect.bottom() + 16.0),
                Vec2::new(40.0, 16.0),
            );
            centered_child(ui, label_rect)
                .label(RichText::new(vol_pct).size(11.0).color(palette.muted));
        });

    volume_changed
}

fn icon_btn_abs(
    ui: &mut egui::Ui,
    palette: Palette,
    icon: ImageSource<'static>,
    rect: Rect,
    primary: bool,
    mut on_click: impl FnMut(),
) {
    let (fill, icon_color, hover) = if primary {
        (palette.accent, palette.accent_text, palette.accent_hover)
    } else {
        (palette.card, palette.foreground, palette.card_hover)
    };

    let response = ui.allocate_rect(rect, egui::Sense::click());
    let bg = if response.hovered() { hover } else { fill };

    ui.painter()
        .rect_filled(rect, if primary { 44.0 } else { 36.0 }, bg);

    centered_child(ui, rect).add(
        egui::Image::new(icon)
            .fit_to_exact_size(Vec2::splat(rect.height() * 0.48))
            .tint(icon_color),
    );

    if response.clicked() {
        on_click();
    }
}

fn format_time(secs: f32) -> String {
    let s = secs.max(0.0).round() as u64;
    format!("{:02}:{:02}", s / 60, s % 60)
}
