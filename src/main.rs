mod app;
mod controller;
mod home;
mod settings;
mod theme;

fn main() -> Result<(), eframe::Error> {
    app::run()
}
