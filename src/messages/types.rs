use crate::views::types::View;

#[derive(Debug)]
pub enum Message {
    GoTo(View),
}
