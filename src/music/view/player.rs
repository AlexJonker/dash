use eframe::egui;
use egui::{
    Align, CornerRadius, Direction, Frame, ImageSource, Pos2, Rect, RichText, UiBuilder, Vec2,
};

use super::super::session::MusicSession;
use crate::theme::Palette;

pub struct MusicChanges {
    pub volume_changed: Option<f32>,
    pub shuffle_changed: Option<bool>,
    pub loop_changed: Option<bool>,
}

#[derive(Clone, Copy)]
enum Icon {
    Previous,
    Play,
    Pause,
    Next,
    MusicNote,
    ShuffleEnabled,
    ShuffleDisabled,
    Volume,
    LoopEnabled,
    LoopDisabled,
}

fn icon(icon: Icon) -> ImageSource<'static> {
    match icon {
        Icon::Previous => egui::include_image!("../../../assets/icons/previous.svg"),
        Icon::Play => egui::include_image!("../../../assets/icons/play.svg"),
        Icon::Pause => egui::include_image!("../../../assets/icons/pause.svg"),
        Icon::Next => egui::include_image!("../../../assets/icons/next.svg"),
        Icon::MusicNote => egui::include_image!("../../../assets/icons/music.svg"),
        Icon::ShuffleEnabled => egui::include_image!("../../../assets/icons/shuffle_enabled.svg"),
        Icon::ShuffleDisabled => egui::include_image!("../../../assets/icons/shuffle_disabled.svg"),
        Icon::Volume => egui::include_image!("../../../assets/icons/volume.svg"),
        Icon::LoopEnabled => egui::include_image!("../../../assets/icons/loop_enabled.svg"),
        Icon::LoopDisabled => egui::include_image!("../../../assets/icons/loop_disabled.svg"),
    }
}

fn centered_child(ui: &mut egui::Ui, rect: Rect) -> egui::Ui {
    ui.new_child(
        UiBuilder::new()
            .max_rect(rect)
            .layout(egui::Layout::centered_and_justified(Direction::LeftToRight)),
    )
}

pub fn show_player(
    ctx: &egui::Context,
    palette: Palette,
    session: &mut MusicSession,
) -> MusicChanges {
    session.tick();
    ctx.request_repaint_after(std::time::Duration::from_millis(16));

    let mut changes = MusicChanges {
        volume_changed: None,
        shuffle_changed: None,
        loop_changed: None,
    };

    egui::CentralPanel::default()
        .frame(Frame::new().fill(palette.background))
        .show(ctx, |ui| {
            let size = ui.available_size();
            let cx = size.x / 2.0;
            let origin = ui.max_rect().min.to_vec2();

            // Sizes
            let cover = (size.y * 0.38).min(260.0);
            let play = 150.0;
            let btn = 80.0;
            let nav = 105.0;

            let small_gap = 16.0;
            let row_w = btn * 2.0 + nav * 2.0 + play + small_gap * 4.0;

            let gaps = (16.0, 14.0, 16.0);
            let meta_h = 96.0;
            let seek_h = 64.0;

            let block_h = cover + gaps.0 + meta_h + gaps.1 + seek_h + gaps.2 + play;
            let cover_top = ((size.y - block_h) / 2.0).max(8.0);

            // Y Positions
            let meta_top = cover_top + cover + gaps.0;
            let seek_top = meta_top + meta_h + gaps.1;
            let ctrl_top = seek_top + seek_h + gaps.2;
            let ctrl_left = cx - row_w / 2.0;
            let ctrl_cy = ctrl_top + play / 2.0;

            // Rects
            let cover_rect =
                Rect::from_center_size(Pos2::new(cx, cover_top + cover / 2.0), Vec2::splat(cover))
                    .translate(origin);

            let title_rect =
                Rect::from_center_size(Pos2::new(cx, meta_top + 18.0), Vec2::new(440.0, 36.0))
                    .translate(origin);
            let artist_rect =
                Rect::from_center_size(Pos2::new(cx, meta_top + 52.0), Vec2::new(440.0, 28.0))
                    .translate(origin);
            let album_rect =
                Rect::from_center_size(Pos2::new(cx, meta_top + 78.0), Vec2::new(440.0, 22.0))
                    .translate(origin);

            let bar_w = (size.x - 60.0).min(520.0);
            let seek_rect =
                Rect::from_center_size(Pos2::new(cx, seek_top + 22.0), Vec2::new(bar_w, 44.0))
                    .translate(origin);
            let time_rect =
                Rect::from_center_size(Pos2::new(cx, seek_top + 54.0), Vec2::new(bar_w, 20.0))
                    .translate(origin);

            let shuffle_rect =
                Rect::from_min_size(Pos2::new(ctrl_left, ctrl_cy - btn / 2.0), Vec2::splat(btn))
                    .translate(origin);

            let prev_rect = Rect::from_min_size(
                Pos2::new(ctrl_left + btn + small_gap, ctrl_cy - nav / 2.0),
                Vec2::splat(nav),
            )
            .translate(origin);

            let play_rect = Rect::from_min_size(
                Pos2::new(ctrl_left + btn + nav + small_gap * 2.0, ctrl_top),
                Vec2::splat(play),
            )
            .translate(origin);

            let next_rect = Rect::from_min_size(
                Pos2::new(
                    ctrl_left + btn + nav + play + small_gap * 3.0,
                    ctrl_cy - nav / 2.0,
                ),
                Vec2::splat(nav),
            )
            .translate(origin);

            let loop_rect = Rect::from_min_size(
                Pos2::new(
                    ctrl_left + btn + nav * 2.0 + play + small_gap * 4.0,
                    ctrl_cy - btn / 2.0,
                ),
                Vec2::splat(btn),
            )
            .translate(origin);

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
                    egui::Image::new(icon(Icon::MusicNote))
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
                    RichText::new("No music found")
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
                const PROGRESS_BAR_HEIGHT: f32 = 20.0;
                let track = Rect::from_center_size(
                    seek_rect.center(),
                    Vec2::new(bar_w, PROGRESS_BAR_HEIGHT),
                );

                painter.rect_filled(track, 4.0, palette.card_hover);

                let fill = (pos / total * bar_w).clamp(0.0, bar_w);
                painter.rect_filled(
                    Rect::from_min_size(track.min, Vec2::new(fill, PROGRESS_BAR_HEIGHT)),
                    4.0,
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
            icon_btn_abs(ui, palette, icon(Icon::Previous), prev_rect, false, || {
                session.previous()
            });

            icon_btn_abs(
                ui,
                palette,
                icon(if session.is_playing() {
                    Icon::Pause
                } else {
                    Icon::Play
                }),
                play_rect,
                true,
                || session.play_pause_toggle(),
            );

            icon_btn_abs(ui, palette, icon(Icon::Next), next_rect, false, || {
                session.next()
            });

            // Shuffle toggle
            let was_shuffle = session.is_shuffle_enabled();

            icon_btn_abs(
                ui,
                palette,
                icon(if session.is_shuffle_enabled() {
                    Icon::ShuffleEnabled
                } else {
                    Icon::ShuffleDisabled
                }),
                shuffle_rect,
                session.is_shuffle_enabled(),
                || {
                    session.shuffle_toggle();
                },
            );

            if session.is_shuffle_enabled() != was_shuffle {
                changes.shuffle_changed = Some(session.is_shuffle_enabled());
            }

            // Loop toggle
            let was_loop = session.is_loop_enabled();

            icon_btn_abs(
                ui,
                palette,
                icon(if session.is_loop_enabled() {
                    Icon::LoopEnabled
                } else {
                    Icon::LoopDisabled
                }),
                loop_rect,
                session.is_loop_enabled(),
                || {
                    session.loop_toggle();
                },
            );

            if session.is_loop_enabled() != was_loop {
                changes.loop_changed = Some(session.is_loop_enabled());
            }

            // Volume slider
            const VOLUME_SLIDER_WIDTH: f32 = 25.0;

            let volume = session.get_volume();

            let vol_icon_size = 22.0;
            let vol_padding = 24.0;

            let bar_h = size.y - vol_padding * 2.0;

            let bar_rect = Rect::from_min_size(
                Pos2::new(27.0, vol_padding),
                Vec2::new(VOLUME_SLIDER_WIDTH, bar_h),
            )
            .translate(origin);

            // Volume bar interaction
            let resp = ui.allocate_rect(bar_rect.expand(10.0), egui::Sense::click_and_drag());

            if resp.clicked() || resp.dragged() {
                let y = resp
                    .interact_pointer_pos()
                    .map(|p| p.y)
                    .unwrap_or(bar_rect.bottom());

                let t = 1.0 - ((y - bar_rect.top()) / bar_rect.height()).clamp(0.0, 1.0);

                session.set_volume(t);
                changes.volume_changed = Some(session.get_volume());
            }

            let painter = ui.painter();

            // Volume bar background
            painter.rect_filled(bar_rect, 4.0, palette.card_hover);

            // Volume bar fill
            let fill = (volume * bar_h).clamp(0.0, bar_h);

            painter.rect_filled(
                Rect::from_min_size(
                    Pos2::new(bar_rect.left(), bar_rect.bottom() - fill),
                    Vec2::new(bar_rect.width(), fill),
                ),
                4.0,
                palette.accent,
            );

            // Volume icon in volume bar
            const VOLUME_ICON_OFFSET: f32 = 12.0;

            let icon_y = (bar_rect.bottom() - fill + VOLUME_ICON_OFFSET).clamp(
                bar_rect.top() + vol_icon_size / 2.0,
                bar_rect.bottom() - vol_icon_size / 2.0,
            );

            let icon_rect = Rect::from_center_size(
                Pos2::new(bar_rect.center().x, icon_y),
                Vec2::splat(vol_icon_size),
            );

            ui.put(
                icon_rect,
                egui::Image::new(icon(Icon::Volume)).fit_to_exact_size(Vec2::splat(vol_icon_size)),
            );
        });

    changes
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
