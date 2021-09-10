use dicetest::hint;
use dicetest::{dice, Die, DieOnce, Fate};
use std::array::IntoIter;
use std::fmt::Debug;
use std::fmt::Write;

use crate::{elem, Elem, Eval};

/// Represents elements that were chosen from a set and that can be used as variables in a
/// predicate.
///
/// It contains `N` elements of type `S`.
#[derive(Clone, Copy)]
pub struct Var<'a, S: Debug, const N: usize> {
    /// The name of the set the elements were chosen from.
    pub set: &'a str,
    /// The elements of the set.
    pub elems: [Elem<'a, S>; N],
}

impl<'a, S: Debug, const N: usize> Var<'a, S, N> {
    pub fn new(set: &'a str, elems: [Elem<'a, S>; N]) -> Self {
        Self { set, elems }
    }

    /// Returns [`Eval`]s that contain the variables.
    ///
    /// This operation will log the variables via [`dicetest::hints`].
    pub fn eval(self) -> [Eval<&'a str, S>; N] {
        let set = self.set;
        let elems = self.elems;

        fn elem_names<S: Debug, const N: usize>(elems: &[Elem<S>; N]) -> String {
            let mut acc = String::new();
            for (i, elem) in elems.iter().enumerate() {
                if i == 0 {
                    write!(acc, "{}", elem.name).unwrap();
                } else {
                    write!(acc, ", {}", elem.name).unwrap();
                }
            }
            acc
        }

        hint!("{} of {}", elem_names(&elems), set);
        elems.map(|elem| elem.eval())
    }

    /// Returns a [`Var`] with the same names and references to the original values.
    pub fn as_ref<'b: 'a>(&'b self) -> Var<'a, &'b S, N> {
        let elems = array_init::array_init(|i| self.elems[i].as_ref());
        Var::new(self.set, elems)
    }
}

pub trait Sealed {}

/// Extension for [`Fate`] that allows to choose variables randomly from a set represented
/// by a generator.
pub trait FateVarExt: Sealed {
    /// Returns a [`Var`] with a single value generated with the given [`DieOnce`].
    fn roll_single_var<'a, S: Debug>(
        &mut self,
        set: &'a str,
        name: &'a str,
        die: impl DieOnce<S>,
    ) -> Var<'a, S, 1>;

    /// Returns a [`Var`] with `N` values generated with the given [`Die`].
    fn roll_var<'a, S: Debug, const N: usize>(
        &mut self,
        set: &'a str,
        names: [&'a str; N],
        die: impl Die<S>,
    ) -> Var<'a, S, N>;
}

impl Sealed for Fate<'_> {}

impl FateVarExt for Fate<'_> {
    fn roll_single_var<'a, S: Debug>(
        &mut self,
        set: &'a str,
        name: &'a str,
        die: impl DieOnce<S>,
    ) -> Var<'a, S, 1> {
        let value = self.roll(die);
        let elem = elem(name, value);
        Var::new(set, [elem])
    }

    fn roll_var<'a, S: Debug, const N: usize>(
        &mut self,
        set: &'a str,
        names: [&'a str; N],
        die: impl Die<S>,
    ) -> Var<'a, S, N> {
        let values = self.roll(dice::array::<_, _, N>(die));
        let elems_iter = IntoIter::new(names)
            .zip(IntoIter::new(values))
            .map(|(name, value)| elem(name, value));
        let elems: [_; N] = array_init::from_iter(elems_iter).unwrap();
        Var::new(set, elems)
    }
}
