mod app;
mod apps;
mod config;
// mod plex; // reserved for a future milestone (Plex integration); intentionally unused in v1
mod platform;
mod ui;

use app::Hub;

pub fn main() -> iced::Result {
    iced::application(Hub::new, Hub::update, Hub::view)
        .title("Hearth")
        .subscription(Hub::subscription)
        .window(iced::window::Settings {
            fullscreen: true,
            ..Default::default()
        })
        .run()
}
