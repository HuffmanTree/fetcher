mod app;
mod components;
mod config;
mod macros;
mod messages;
mod projects;
mod requests;
mod views;

use crate::app::types::App;
use iced::Theme;

fn main() -> iced::Result {
    iced::application(App::boot, App::update, App::view)
        .theme(Theme::Dark)
        .run()
}
