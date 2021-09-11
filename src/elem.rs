use dicetest::hint;
use std::fmt::Debug;

use crate::Eval;

/// Represents an element of type `S`.
#[derive(Clone, Copy)]
pub struct Elem<'a, S: Debug> {
    /// The human-readable name of the element.
    pub name: &'a str,
    value: S,
}

impl<'a, S: Debug> Elem<'a, S> {
    pub fn new(name: &'a str, value: S) -> Self {
        Self { name, value }
    }

    /// Returns an [`Eval`] that contains this element.
    ///
    /// This operation will log the element via [`dicetest::hints`].
    pub fn eval(self) -> Eval<&'a str, S> {
        hint!("{} = {:?}", self.name, self.value);

        Eval {
            label: self.name,
            value: self.value,
        }
    }

    /// Returns an [`Elem`] with the same name and a reference to the original value.
    pub fn as_ref(&self) -> Elem<&S> {
        Elem::new(self.name, &self.value)
    }
}

impl<'a, 'b, S: Debug> Elem<'a, &'b S> {
    /// Returns an [`Elem`] with the same name and a clone of the original value.
    pub fn cloned(self) -> Elem<'a, S>
    where
        S: Clone,
    {
        Elem::new(self.name, self.value.clone())
    }

    /// Returns an [`Elem`] with the same name and a copy of the original value.
    pub fn copied(self) -> Elem<'a, S>
    where
        S: Copy,
    {
        Elem::new(self.name, *self.value)
    }
}
