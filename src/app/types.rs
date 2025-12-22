use crate::views::types::View;

#[derive(Default)]
pub struct App {
    pub current_view: Option<View>,
}
