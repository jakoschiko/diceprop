use dicetest::hint;
use std::fmt::{self, Debug, Display};

use crate::Eval;

#[derive(Clone, Copy)]
pub enum Fun1Syntax {
    Prefix,
    Postfix,
}

/// A human-readable label that describes a [`Fun1`] applied to an argument.
#[derive(Clone, Copy)]
pub struct Fun1Label<'a, AL1: Display + Copy> {
    fun_name: &'a str,
    fun_syntax: Fun1Syntax,
    arg_1_label: AL1,
}

impl<'a, AL1: Display + Copy> Display for Fun1Label<'a, AL1> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.fun_syntax {
            Fun1Syntax::Prefix => write!(f, "{}({})", self.fun_name, self.arg_1_label),
            Fun1Syntax::Postfix => write!(f, "({}){}", self.arg_1_label, self.fun_name),
        }
    }
}

/// Represents a function of arity 1.
pub struct Fun1<'a, F> {
    /// The name of the function.
    pub name: &'a str,
    syntax: Fun1Syntax,
    f: F,
}

impl<'a, F> Fun1<'a, F> {
    /// Creates a [`Fun1`] with the given name and prefix syntax (e.g. `f(x)`).
    pub fn new(name: &'a str, f: F) -> Self {
        let syntax = Fun1Syntax::Prefix;
        Self { name, syntax, f }
    }

    /// Creates a [`Fun1`] with the given name and postfix syntax (e.g. `(x)!`).
    pub fn postfix(name: &'a str, f: F) -> Self {
        let syntax = Fun1Syntax::Postfix;
        Self { name, syntax, f }
    }

    fn label<AL1>(&self, arg_1_label: AL1) -> Fun1Label<'a, AL1>
    where
        AL1: Display + Copy,
    {
        Fun1Label {
            fun_name: self.name,
            fun_syntax: self.syntax,
            arg_1_label,
        }
    }

    /// Returns an [`Eval`] that contains the result of the function applied to the given
    /// argument.
    ///
    /// This operation will log the function application via [`dicetest::hints`].
    pub fn eval<AL1, AV1, RV>(&self, arg_1: Eval<AL1, AV1>) -> Eval<Fun1Label<'a, AL1>, RV>
    where
        AL1: Display + Copy,
        RV: Debug,
        F: Fn(AV1) -> RV,
    {
        let res_label = self.label(arg_1.label);
        let res_value = (self.f)(arg_1.value);

        hint!("{} = {:?}", res_label, res_value);

        Eval {
            label: res_label,
            value: res_value,
        }
    }

    /// Returns an [`Eval`] that contains the result of the function applied to the given
    /// argument.
    ///
    /// This operation will log the function application via [`dicetest::hints`].
    pub fn eval_once<AL1, AV1, RV>(self, arg_1: Eval<AL1, AV1>) -> Eval<Fun1Label<'a, AL1>, RV>
    where
        AL1: Display + Copy,
        RV: Debug,
        F: FnOnce(AV1) -> RV,
    {
        let res_label = self.label(arg_1.label);
        let res_value = (self.f)(arg_1.value);

        hint!("{} = {:?}", res_label, res_value);

        Eval {
            label: res_label,
            value: res_value,
        }
    }

    /// Returns an [`Fun1`] with the same name and syntax and a reference to the original
    /// function.
    pub fn as_ref<'b>(&'b self) -> Fun1<'a, &'b F> {
        Fun1 {
            name: self.name,
            syntax: self.syntax,
            f: &self.f,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Fun2Syntax {
    Prefix,
    Infix,
}

/// A human-readable label that describes a [`Fun2`] applied to arguments.
#[derive(Clone, Copy)]
pub struct Fun2Label<'a, AL1: Display + Copy, AL2: Display + Copy> {
    fun_name: &'a str,
    fun_syntax: Fun2Syntax,
    arg_1_label: AL1,
    arg_2_label: AL2,
}

impl<'a, AL1: Display + Copy, AL2: Display + Copy> Display for Fun2Label<'a, AL1, AL2> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.fun_syntax {
            Fun2Syntax::Prefix => write!(
                f,
                "{}({}, {})",
                self.fun_name, self.arg_1_label, self.arg_2_label
            ),
            Fun2Syntax::Infix => write!(
                f,
                "({} {} {})",
                self.arg_1_label, self.fun_name, self.arg_2_label
            ),
        }
    }
}

/// Represents a function of arity 2.
pub struct Fun2<'a, F> {
    /// The name of the function.
    pub name: &'a str,
    syntax: Fun2Syntax,
    f: F,
}

impl<'a, F> Fun2<'a, F> {
    /// Creates a [`Fun2`] with the given name and prefix syntax (e.g. `f(x, y)`).
    pub fn new(name: &'a str, f: F) -> Self {
        let syntax = Fun2Syntax::Prefix;
        Self { name, syntax, f }
    }

    /// Creates a [`Fun2`] with the given name and infix syntax (e.g. `(x + y)`).
    pub fn infix(name: &'a str, f: F) -> Self {
        let syntax = Fun2Syntax::Infix;
        Self { name, syntax, f }
    }

    fn label<AL1, AL2>(&self, arg_1_label: AL1, arg_2_label: AL2) -> Fun2Label<'a, AL1, AL2>
    where
        AL1: Display + Copy,
        AL2: Display + Copy,
    {
        Fun2Label {
            fun_name: self.name,
            fun_syntax: self.syntax,
            arg_1_label,
            arg_2_label,
        }
    }

    /// Returns an [`Eval`] that contains the result of the function applied to the given
    /// arguments.
    ///
    /// This operation will log the function application via [`dicetest::hints`].
    pub fn eval<AL1, AL2, AV1, AV2, RV>(
        &self,
        arg_1: Eval<AL1, AV1>,
        arg_2: Eval<AL2, AV2>,
    ) -> Eval<Fun2Label<'a, AL1, AL2>, RV>
    where
        AL1: Display + Copy,
        AL2: Display + Copy,
        RV: Debug,
        F: Fn(AV1, AV2) -> RV,
    {
        let res_label = self.label(arg_1.label, arg_2.label);
        let res_value = (self.f)(arg_1.value, arg_2.value);

        hint!("{} = {:?}", res_label, res_value);

        Eval {
            label: res_label,
            value: res_value,
        }
    }

    /// Returns an [`Eval`] that contains the result of the function applied to the given
    /// arguments.
    ///
    /// This operation will log the function application via [`dicetest::hints`].
    pub fn eval_once<AL1, AL2, AV1, AV2, RV>(
        self,
        arg_1: Eval<AL1, AV1>,
        arg_2: Eval<AL2, AV2>,
    ) -> Eval<Fun2Label<'a, AL1, AL2>, RV>
    where
        AL1: Display + Copy,
        AL2: Display + Copy,
        RV: Debug,
        F: FnOnce(AV1, AV2) -> RV,
    {
        let res_label = self.label(arg_1.label, arg_2.label);
        let res_value = (self.f)(arg_1.value, arg_2.value);

        hint!("{} = {:?}", res_label, res_value);

        Eval {
            label: res_label,
            value: res_value,
        }
    }

    /// Returns an [`Fun2`] with the same name and syntax and a reference to the original
    /// function.
    pub fn as_ref<'b>(&'b self) -> Fun2<'a, &'b F> {
        Fun2 {
            name: self.name,
            syntax: self.syntax,
            f: &self.f,
        }
    }
}

/// A human-readable label that describes a [`Fun3`] applied to arguments.
#[derive(Clone, Copy)]
pub struct Fun3Label<'a, AL1: Display + Copy, AL2: Display + Copy, AL3: Display + Copy> {
    fun_name: &'a str,
    arg_1_label: AL1,
    arg_2_label: AL2,
    arg_3_label: AL3,
}

impl<'a, AL1: Display + Copy, AL2: Display + Copy, AL3: Display + Copy> Display
    for Fun3Label<'a, AL1, AL2, AL3>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}({}, {}, {})",
            self.fun_name, self.arg_1_label, self.arg_2_label, self.arg_3_label
        )
    }
}

/// Represents a function of arity 3.
pub struct Fun3<'a, F> {
    /// The name of the function.
    pub name: &'a str,
    f: F,
}

impl<'a, F> Fun3<'a, F> {
    /// Creates a [`Fun3`] with the given name and prefix syntax (e.g. `f(x, y, z)`).
    pub fn new(name: &'a str, f: F) -> Self {
        Self { name, f }
    }

    fn label<AL1, AL2, AL3>(
        &self,
        arg_1_label: AL1,
        arg_2_label: AL2,
        arg_3_label: AL3,
    ) -> Fun3Label<'a, AL1, AL2, AL3>
    where
        AL1: Display + Copy,
        AL2: Display + Copy,
        AL3: Display + Copy,
    {
        Fun3Label {
            fun_name: self.name,
            arg_1_label,
            arg_2_label,
            arg_3_label,
        }
    }

    /// Returns an [`Eval`] that contains the result of the function applied to the given
    /// arguments.
    ///
    /// This operation will log the function application via [`dicetest::hints`].
    pub fn eval<AL1, AL2, AL3, AV1, AV2, AV3, RV>(
        &self,
        arg_1: Eval<AL1, AV1>,
        arg_2: Eval<AL2, AV2>,
        arg_3: Eval<AL3, AV3>,
    ) -> Eval<Fun3Label<'a, AL1, AL2, AL3>, RV>
    where
        AL1: Display + Copy,
        AL2: Display + Copy,
        AL3: Display + Copy,
        RV: Debug,
        F: Fn(AV1, AV2, AV3) -> RV,
    {
        let res_label = self.label(arg_1.label, arg_2.label, arg_3.label);
        let res_value = (self.f)(arg_1.value, arg_2.value, arg_3.value);

        hint!("{} = {:?}", res_label, res_value);

        Eval {
            label: res_label,
            value: res_value,
        }
    }

    /// Returns an [`Eval`] that contains the result of the function applied to the given
    /// arguments.
    ///
    /// This operation will log the function application via [`dicetest::hints`].
    pub fn eval_once<AL1, AL2, AL3, AV1, AV2, AV3, RV>(
        self,
        arg_1: Eval<AL1, AV1>,
        arg_2: Eval<AL2, AV2>,
        arg_3: Eval<AL3, AV3>,
    ) -> Eval<Fun3Label<'a, AL1, AL2, AL3>, RV>
    where
        AL1: Display + Copy,
        AL2: Display + Copy,
        AL3: Display + Copy,
        RV: Debug,
        F: FnOnce(AV1, AV2, AV3) -> RV,
    {
        let res_label = self.label(arg_1.label, arg_2.label, arg_3.label);
        let res_value = (self.f)(arg_1.value, arg_2.value, arg_3.value);

        hint!("{} = {:?}", res_label, res_value);

        Eval {
            label: res_label,
            value: res_value,
        }
    }

    /// Returns an [`Fun3`] with the same name and syntax and a reference to the original
    /// function.
    pub fn as_ref<'b>(&'b self) -> Fun3<'a, &'b F> {
        Fun3 {
            name: self.name,
            f: &self.f,
        }
    }
}
