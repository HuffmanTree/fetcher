mod app;
mod macros;
mod messages;
mod views;

use crate::app::types::App;

fn main() -> iced::Result {
    iced::application(App::boot, App::update, App::view).run()
}
