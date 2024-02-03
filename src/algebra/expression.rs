use std::any::Any;
use std::fmt::Debug;

use dyn_clone::DynClone;

pub trait Expression: DynClone {
    fn eval(&self) -> Box<dyn Expression>;
    fn simplify(&self) -> Box<dyn Expression>;
    fn as_any(&self) -> &dyn Any;

    fn debug(&self, indent: usize) -> String;
    fn to_typist(&self) -> String;
}

dyn_clone::clone_trait_object!(Expression);

trait AsAny {
    fn as_any(&self) -> &dyn Any;
}

impl<T: Expression + 'static> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Debug for dyn Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.debug(0))
    }
}
