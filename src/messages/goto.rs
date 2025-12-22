use crate::app::types::App;
use crate::views::types::View;

pub trait GoTo {
    fn goto(&mut self, view: View);
}

impl GoTo for App {
    fn goto(&mut self, view: View) {
        self.current_view = Some(view);
    }
}

#[cfg(test)]
mod tests {
    use crate::app::types::App;
    use crate::messages::goto::GoTo;
    use crate::views::types::View;

    #[test]
    fn goto_projects() {
        let mut app = App::default();

        app.goto(View::Projects);

        assert_eq!(app.current_view, Some(View::Projects));
    }
}
