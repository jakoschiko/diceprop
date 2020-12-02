use std::fmt::Display;

/// Represents a evaluated expression of type `S`.
#[derive(Clone, Copy)]
pub struct Eval<L: Display + Copy, S> {
    /// A human-readable label that describes the expression that was evaluated
    /// (e.g. `"x"` or `"f(x, y)"`).
    pub label: L,
    /// The value of the evaluated expression.
    pub value: S,
}

impl<L: Display + Copy, S> Eval<L, S> {
    /// Returns an [`Eval`] with the same label and a reference to the original value.
    pub fn as_ref(&self) -> Eval<L, &S> {
        Eval {
            label: self.label,
            value: &self.value,
        }
    }
}

impl<'a, L: Display + Copy, S> Eval<L, &'a S> {
    /// Returns an [`Eval`] with the same label and a clone of the original value.
    pub fn cloned(self) -> Eval<L, S>
    where
        S: Clone,
    {
        Eval {
            label: self.label,
            value: self.value.clone(),
        }
    }

    /// Returns an [`Eval`] with the same label and a copy of the original value.
    pub fn copied(self) -> Eval<L, S>
    where
        S: Copy,
    {
        Eval {
            label: self.label,
            value: *self.value,
        }
    }
}
