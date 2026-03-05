mod pages;
mod theme;

use std::time::Duration;

use eframe::egui;

use pages::home::HomeAction;
use pages::settings::SettingsOutcome;
use theme::{Palette, apply_style};

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "Miata Dash",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Clone, Copy, PartialEq)]
enum AppView {
    Home,
    Settings,
}

struct MyApp {
    dark_mode: bool,
    view: AppView,
    current_palette: Palette,
    style_applied: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        let palette = Palette::dark();

        Self {
            dark_mode: true,
            view: AppView::Home,
            current_palette: palette,
            style_applied: false,
        }
    }
}

impl MyApp {
    fn palette(&self) -> Palette {
        if self.dark_mode {
            Palette::dark()
        } else {
            Palette::light()
        }
    }

    fn ensure_style(&mut self, ctx: &egui::Context, palette: Palette) {
        if !self.style_applied || self.current_palette != palette {
            apply_style(ctx, palette);
            self.current_palette = palette;
            self.style_applied = true;
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut palette = self.palette();
        self.ensure_style(ctx, palette);

        let mut theme_changed = false;

        match self.view {
            AppView::Home => {
                if let Some(HomeAction::OpenSettings) = pages::home::show(ctx, palette) {
                    self.view = AppView::Settings;
                }
            }
            AppView::Settings => {
                let outcome: SettingsOutcome = pages::settings::show(ctx, palette, &mut self.dark_mode);
                theme_changed |= outcome.theme_changed;

                if outcome.go_home {
                    self.view = AppView::Home;
                }
            }
        }

        if theme_changed {
            palette = self.palette();
            self.ensure_style(ctx, palette);
        }

        ctx.request_repaint_after(Duration::from_millis(500));
    }
}
