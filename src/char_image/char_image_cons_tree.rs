use cons::{
    cell::Cons,
    single::Single,
    tree::{Branch, Leaf},
};

use super::CharImage;

/// A `ConsTree` of `CharImage` types
pub trait CharImageConsTree<I> {}

impl<'a, L, R, CAR, CDR> CharImageConsTree<Branch<L, R>> for Cons<CAR, CDR>
where
    CAR: CharImageConsTree<L>,
    CDR: CharImageConsTree<R>,
{
}

impl<'a, CAR> CharImageConsTree<Leaf> for Single<CAR> where CAR: CharImage {}
