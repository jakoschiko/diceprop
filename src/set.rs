use dicetest::{dice, Die, DieOnce};
use std::array::IntoIter;
use std::fmt::Debug;
use std::marker::PhantomData;

use crate::{elem, Var};

/// A mathematical set that can be used to choose variables.
///
/// The elements of the set are represented by the generator. The set contains all elements
/// that the generator could possibly generate.
pub struct Set<'a, S: Debug, D: DieOnce<S> + 'a> {
    /// The name of the set.
    pub name: &'a str,
    /// A generator for the elements of the set.
    pub elem_die: D,
    _s: PhantomData<S>,
}

impl<'a, S: Debug, D: DieOnce<S> + 'a> Set<'a, S, D> {
    pub fn new(name: &'a str, elem_die: D) -> Self {
        Self {
            name,
            elem_die,
            _s: PhantomData,
        }
    }

    /// Returns a generator that chooses a single variable from the set.
    pub fn single_var(self, name: &'a str) -> impl DieOnce<Var<'a, S, 1>> {
        let set = self.name;
        let die = self.elem_die;
        dice::from_fn_once(move |mut fate| {
            let value = fate.roll(die);
            let elem = elem(name, value);
            Var::new(set, [elem])
        })
    }
}

impl<'a, S: Debug, D: Die<S> + 'a> Set<'a, S, D> {
    /// Returns a [`Set`] with the same name and a reference to the original generator.
    pub fn as_ref<'b: 'a>(&'b self) -> Set<'a, S, &'b D> {
        Set {
            name: self.name,
            elem_die: &self.elem_die,
            _s: PhantomData,
        }
    }

    /// Returns a generator that chooses `N` variables from the set.
    pub fn var<const N: usize>(self, names: [&'a str; N]) -> impl Die<Var<'a, S, N>> {
        let set = self.name;
        let die = self.elem_die;
        dice::from_fn(move |mut fate| {
            let values = fate.roll(dice::array::<_, _, N>(&die));
            let elems_iter = IntoIter::new(names)
                .zip(IntoIter::new(values))
                .map(|(name, value)| elem(name, value));
            let elems: [_; N] = array_init::from_iter(elems_iter).unwrap();
            Var::new(set, elems)
        })
    }
}
