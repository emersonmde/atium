use std::any::Any;

use dyn_clone::DynClone;

pub trait Expression: DynClone + std::fmt::Debug {
    fn eval(&self) -> Box<dyn Expression>;
    fn simplify(&self) -> Box<dyn Expression>;
    fn as_any(&self) -> &dyn Any;
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

