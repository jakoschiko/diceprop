#[macro_use]
mod eval;
pub use eval::Eval;

mod elem;
pub use elem::{elem, Elem};

mod var;
pub use var::{var_1, var_2, var_3, FateVarExt, Var1, Var2, Var3};

mod fun;
pub use fun::{fun_1, fun_2, infix_fun_2, postfix_fun_1, Fun1, Fun1Label, Fun2, Fun2Label};

pub mod ops;

pub mod props;
