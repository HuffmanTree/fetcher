pub mod project;
pub mod request;

pub trait Mutation {
    type Item;

    fn apply(self, item: &mut Self::Item);
}
