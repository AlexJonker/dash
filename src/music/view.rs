use eframe::egui;
use egui::{
    Align, CornerRadius, Direction, Frame, ImageSource, Pos2, Rect, RichText, UiBuilder, Vec2,
};

use super::session::MusicSession;
use crate::theme::Palette;

#[derive(Clone, Copy)]
enum ControlIcon {
    Shuffle,
    Previous,
    Play,
    Pause,
    Next,
}

fn control_icon_source(icon: ControlIcon) -> ImageSource<'static> {
    match icon {
        ControlIcon::Shuffle => egui::include_image!("../../assets/icons/shuffle.svg"),
        ControlIcon::Previous => egui::include_image!("../../assets/icons/previous.svg"),
        ControlIcon::Play => egui::include_image!("../../assets/icons/play.svg"),
        ControlIcon::Pause => egui::include_image!("../../assets/icons/pause.svg"),
        ControlIcon::Next => egui::include_image!("../../assets/icons/next.svg"),
    }
}

pub fn show(ctx: &egui::Context, palette: Palette, session: &mut MusicSession) {
    session.tick();
    ctx.request_repaint_after(std::time::Duration::from_millis(16));

    egui::CentralPanel::default()
        .frame(Frame::new().fill(palette.background))
        .show(ctx, |ui| {
            let total_w = ui.available_width();
            let total_h = ui.available_height();

            ui.vertical_centered(|ui| {
                ui.set_max_width(total_w);

                ui.add_space(28.0);

                let cover = (total_h * 0.38).min(260.0);
                if let Some(tex) = session.current_cover(ctx) {
                    ui.add(
                        egui::Image::new(tex)
                            .fit_to_exact_size(Vec2::splat(cover))
                            .corner_radius(CornerRadius::same(16)),
                    );
                } else {
                    let (rect, _) =
                        ui.allocate_exact_size(Vec2::splat(cover), egui::Sense::hover());
                    ui.painter()
                        .rect_filled(rect, CornerRadius::same(16), palette.card);
                    ui.painter().text(
                        rect.center(),
                        egui::Align2::CENTER_CENTER,
                        "♪",
                        egui::FontId::proportional(64.0),
                        palette.muted,
                    );
                }

                ui.add_space(16.0);

                if let Some(track) = session.current_track() {
                    ui.label(
                        RichText::new(&track.title)
                            .size(28.0)
                            .strong()
                            .color(palette.foreground),
                    );
                    ui.add_space(2.0);
                    ui.label(RichText::new(&track.artist).size(20.0).color(palette.muted));
                    ui.add_space(2.0);
                    ui.label(RichText::new(&track.album).size(16.0).color(palette.muted));
                } else {
                    ui.label(
                        RichText::new("Press Shuffle to start")
                            .size(24.0)
                            .color(palette.muted),
                    );
                }

                ui.add_space(20.0);

                if let Some(total) = session.current_duration_secs() {
                    let mut pos = session.current_position_secs().min(total);
                    let bar_w = (total_w - 60.0).min(520.0);
                    let bar_h = 8.0;
                    let handle_r = 10.0;

                    let (rect, resp) =
                        ui.allocate_exact_size(Vec2::new(bar_w, 44.0), egui::Sense::drag());

                    if resp.dragged() || resp.drag_stopped() {
                        let x = resp
                            .interact_pointer_pos()
                            .map(|p| p.x)
                            .unwrap_or(rect.left());
                        let t = ((x - rect.left()) / rect.width()).clamp(0.0, 1.0);
                        pos = t * total;
                        session.seek_to_secs(pos);
                    }

                    let painter = ui.painter();
                    let bar_rect = Rect::from_min_size(
                        Pos2::new(rect.left(), rect.center().y - bar_h / 2.0),
                        Vec2::new(bar_w, bar_h),
                    );

                    painter.rect_filled(bar_rect, CornerRadius::same(4), palette.card_hover);

                    let fill_w = (pos / total * bar_w).clamp(0.0, bar_w);
                    if fill_w > 0.0 {
                        painter.rect_filled(
                            Rect::from_min_size(bar_rect.min, Vec2::new(fill_w, bar_h)),
                            CornerRadius::same(4),
                            palette.accent,
                        );
                    }

                    let handle_x = rect.left() + fill_w;
                    painter.circle_filled(
                        Pos2::new(handle_x, rect.center().y),
                        handle_r,
                        palette.accent,
                    );

                    ui.allocate_ui_with_layout(
                        Vec2::new(bar_w, 20.0),
                        egui::Layout::left_to_right(Align::Center),
                        |ui| {
                            ui.label(
                                RichText::new(format_time(pos))
                                    .size(14.0)
                                    .color(palette.muted),
                            );
                            ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                                ui.label(
                                    RichText::new(format_time(total))
                                        .size(14.0)
                                        .color(palette.muted),
                                );
                            });
                        },
                    );
                }
                ui.add_space(20.0);

                ui.horizontal(|ui| {
                    let btn_w = 64.0;
                    let play_w = 80.0;
                    let gap = 20.0;

                    let row_w = btn_w * 2.0 + play_w + gap * 2.0;
                    let padding = ((total_w - row_w) / 2.0).max(0.0);
                    ui.add_space(padding);

                    // icon_btn(
                    //     ui,
                    //     palette,
                    //     control_icon_source(ControlIcon::Shuffle),
                    //     btn_w,
                    //     btn_w,
                    //     false,
                    //     || session.shuffle_all(),
                    // );
                    // ui.add_space(gap);
                    icon_btn(
                        ui,
                        palette,
                        control_icon_source(ControlIcon::Previous),
                        btn_w,
                        btn_w,
                        false,
                        || session.previous(),
                    );
                    ui.add_space(gap);
                    icon_btn(
                        ui,
                        palette,
                        control_icon_source(if session.is_playing() {
                            ControlIcon::Pause
                        } else {
                            ControlIcon::Play
                        }),
                        play_w,
                        play_w,
                        true,
                        || session.play_pause_toggle(),
                    );
                    ui.add_space(gap);
                    icon_btn(
                        ui,
                        palette,
                        control_icon_source(ControlIcon::Next),
                        btn_w,
                        btn_w,
                        false,
                        || session.next(),
                    );
                });
            });
        });
}

fn icon_btn(
    ui: &mut egui::Ui,
    palette: Palette,
    icon: ImageSource<'static>,
    w: f32,
    h: f32,
    primary: bool,
    mut on_click: impl FnMut(),
) {
    let (fill, icon_color, hover_fill) = if primary {
        (palette.accent, palette.accent_text, palette.accent_hover)
    } else {
        (palette.card, palette.foreground, palette.card_hover)
    };

    let rounding = CornerRadius::same(if primary { 44 } else { 36 });

    let (rect, response) = ui.allocate_exact_size(Vec2::new(w, h), egui::Sense::click());
    if ui.is_rect_visible(rect) {
        let mut child = ui.new_child(
            UiBuilder::new()
                .max_rect(rect)
                .layout(egui::Layout::centered_and_justified(Direction::LeftToRight)),
        );

        let bg = if response.hovered() { hover_fill } else { fill };
        child.painter().rect_filled(rect, rounding, bg);

        child.add(
            egui::Image::new(icon)
                .fit_to_exact_size(Vec2::splat(h * 0.48))
                .tint(icon_color),
        );
    }

    if response.clicked() {
        on_click();
    }
}

fn format_time(secs: f32) -> String {
    let s = secs.max(0.0).round() as u64;
    format!("{:02}:{:02}", s / 60, s % 60)
}
