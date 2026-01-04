use crate::mutations::Mutation;
use crate::requests::types::Request;

#[derive(Clone, Debug)]
pub enum RequestMutation {
    Url(String),
}

impl Mutation for RequestMutation {
    type Item = Request;

    fn apply(self, item: &mut Self::Item) {
        match self {
            RequestMutation::Url(url) => item.set_url(url),
        }
    }
}
