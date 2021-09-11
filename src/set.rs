use dicetest::{Die, DieOnce};
use std::marker::PhantomData;

/// A mathematical set that can be used to choose variables.
///
/// The elements of the set are represented by the generator. The set contains all elements
/// that the generator could possibly generate.
pub struct Set<'a, S, D: DieOnce<S> + 'a> {
    /// The name of the set.
    pub name: &'a str,
    /// A generator for the elements of the set.
    pub elem_die: D,
    _s: PhantomData<S>,
}

impl<'a, S, D: DieOnce<S> + 'a> Set<'a, S, D> {
    pub fn new(name: &'a str, elem_die: D) -> Self {
        Self {
            name,
            elem_die,
            _s: PhantomData,
        }
    }
}

impl<'a, S, D: Die<S> + 'a> Set<'a, S, D> {
    /// Returns a [`Set`] with the same name and a reference to the original generator.
    pub fn as_ref<'b: 'a>(&'b self) -> Set<'a, S, &'b D> {
        Set {
            name: self.name,
            elem_die: &self.elem_die,
            _s: PhantomData,
        }
    }
}
