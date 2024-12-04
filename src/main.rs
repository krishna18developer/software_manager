mod app;
mod models;

use app::SoftwareManager;
use iced::{Application, Settings};

fn main() -> iced::Result {
    // Initialize logging
    env_logger::init();

    // Start the application
    SoftwareManager::run(Settings::default())
}
