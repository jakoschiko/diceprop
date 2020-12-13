use dicetest::hint;
use dicetest::{dice, Die, DieOnce, Fate};
use std::fmt::Debug;

use crate::{elem, Elem, Eval};

/// Represents an element of set `S` that can be used as a variable in a predicate.
#[derive(Clone, Copy)]
pub struct Var1<'a, S: Debug> {
    /// The name of the set the element was taken from.
    pub set: &'a str,
    /// The element of the set.
    pub elem: Elem<'a, S>,
}

impl<'a, S: Debug> Var1<'a, S> {
    /// Returns an [`Eval`] that contains the variable.
    ///
    /// This operation will log the variable via [`dicetest::hints`].
    pub fn eval(self) -> Eval<&'a str, S> {
        let set = self.set;
        let elem = self.elem;
        hint!("{} of {}", elem.name, set);
        elem.eval()
    }

    /// Returns a [`Var1`] with the same name and a reference to the original value.
    pub fn as_ref<'b: 'a>(&'b self) -> Var1<'a, &'b S> {
        var_1(self.set, self.elem.as_ref())
    }
}

/// Creates a [`Var1`] with the given element `elem` taken from a set with name `set`.
pub fn var_1<'a, S: Debug>(set: &'a str, elem: Elem<'a, S>) -> Var1<'a, S> {
    Var1 { set, elem }
}

/// Represents 2 elements of set `S` that can be used as variables in a predicate.
#[derive(Clone, Copy)]
pub struct Var2<'a, S: Debug> {
    /// The name of the set the elements were taken from.
    pub set: &'a str,
    /// The elements of the set.
    pub elems: [Elem<'a, S>; 2],
}

impl<'a, S: Debug> Var2<'a, S> {
    /// Returns [`Eval`]s that contain the variables.
    ///
    /// This operation will log the variables via [`dicetest::hints`].
    pub fn eval(self) -> [Eval<&'a str, S>; 2] {
        let set = self.set;
        let [elem0, elem1] = self.elems;
        hint!("{}, {} of {}", elem0.name, elem1.name, set);
        [elem0.eval(), elem1.eval()]
    }

    /// Returns a [`Var2`] with the same names and references to the original values.
    pub fn as_ref<'b: 'a>(&'b self) -> Var2<'a, &'b S> {
        var_2(self.set, [self.elems[0].as_ref(), self.elems[1].as_ref()])
    }
}

/// Creates a [`Var2`] with the given elements `elems` taken from a set with name `set`.
pub fn var_2<'a, S: Debug>(set: &'a str, elems: [Elem<'a, S>; 2]) -> Var2<'a, S> {
    Var2 { set, elems }
}

/// Represents 3 elements of set `S` that can be used as variables in a predicate.
#[derive(Clone, Copy)]
pub struct Var3<'a, S: Debug> {
    /// The name of the set the elements were taken from.
    pub set: &'a str,
    /// The elements of the set.
    pub elems: [Elem<'a, S>; 3],
}

impl<'a, S: Debug> Var3<'a, S> {
    /// Returns [`Eval`]s that contain the variables.
    ///
    /// This operation will log the variables via [`dicetest::hints`].
    pub fn eval(self) -> [Eval<&'a str, S>; 3] {
        let set = self.set;
        let [elem0, elem1, elem2] = self.elems;
        hint!("{}, {}, {} of {}", elem0.name, elem1.name, elem2.name, set,);
        [elem0.eval(), elem1.eval(), elem2.eval()]
    }

    /// Returns a [`Var3`] with the same names and references to the original values.
    pub fn as_ref<'b: 'a>(&'b self) -> Var3<'a, &'b S> {
        var_3(
            self.set,
            [
                self.elems[0].as_ref(),
                self.elems[1].as_ref(),
                self.elems[2].as_ref(),
            ],
        )
    }
}

/// Creates a [`Var3`] with the given elements `elems` taken from a set with name `set`.
pub fn var_3<'a, S: Debug>(set: &'a str, elems: [Elem<'a, S>; 3]) -> Var3<'a, S> {
    Var3 { set, elems }
}

pub trait Sealed {}

/// Extension for [`Fate`] that allows to "pick" variables randomly from a set represented
/// by a generator.
pub trait FateVarExt: Sealed {
    /// Returns a [`Var1`] with a value generated with the given [`DieOnce`].
    fn roll_var_1<'a, S: Debug>(
        &mut self,
        set: &'a str,
        name: &'a str,
        die: impl DieOnce<S>,
    ) -> Var1<'a, S>;

    /// Returns a [`Var2`] with values generated with the given [`Die`].
    fn roll_var_2<'a, S: Debug>(
        &mut self,
        set: &'a str,
        names: [&'a str; 2],
        die: impl Die<S>,
    ) -> Var2<'a, S>;

    /// Returns a [`Var3`] with values generated with the given [`Die`].
    fn roll_var_3<'a, S: Debug>(
        &mut self,
        set: &'a str,
        names: [&'a str; 3],
        die: impl Die<S>,
    ) -> Var3<'a, S>;
}

impl Sealed for Fate<'_> {}

impl FateVarExt for Fate<'_> {
    fn roll_var_1<'a, S: Debug>(
        &mut self,
        set: &'a str,
        name: &'a str,
        die: impl DieOnce<S>,
    ) -> Var1<'a, S> {
        let value = self.roll(die);
        let elem = elem(name, value);
        var_1(set, elem)
    }

    fn roll_var_2<'a, S: Debug>(
        &mut self,
        set: &'a str,
        names: [&'a str; 2],
        die: impl Die<S>,
    ) -> Var2<'a, S> {
        let [name1, name2] = names;
        let [value1, value2] = self.roll(dice::array_2(die));
        let elems = [elem(name1, value1), elem(name2, value2)];
        var_2(set, elems)
    }

    fn roll_var_3<'a, S: Debug>(
        &mut self,
        set: &'a str,
        names: [&'a str; 3],
        die: impl Die<S>,
    ) -> Var3<'a, S> {
        let [name1, name2, name3] = names;
        let [value1, value2, value3] = self.roll(dice::array_3(die));
        let elems = [
            elem(name1, value1),
            elem(name2, value2),
            elem(name3, value3),
        ];
        var_3(set, elems)
    }
}
