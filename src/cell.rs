use crate::{IntoNoun, Noun};

pub trait Cell: IntoNoun + Sized {
    type Noun: Noun;

    fn get(&self, idx: usize) -> Option<<Self as Cell>::Noun>;

    fn into_parts(self) -> (Option<<Self as Cell>::Noun>, Option<<Self as Cell>::Noun>);
}
