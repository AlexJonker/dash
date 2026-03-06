mod app;
mod controller;
mod home;
mod music;
mod settings;
mod theme;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn main() -> Result<(), eframe::Error> {
    app::run()
}
