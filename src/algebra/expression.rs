use dyn_clone::DynClone;
use std::any::Any;
use std::fmt::Debug;

/// The `Expression` trait represents an algebraic expression.
/// It provides methods for evaluating and simplifying the expression,
/// as well as converting the expression to a debug string or a Typist string.
pub trait Expression: DynClone {
    /// Evaluates the expression and returns a new expression.
    fn eval(&self) -> Box<dyn Expression>;

    /// Simplifies the expression and returns a new simplified expression.
    fn simplify(&self) -> Box<dyn Expression>;

    /// Returns a reference to the expression as a `dyn Any`, which can be downcast to its concrete type.
    fn as_any(&self) -> &dyn Any;

    /// Returns a debug string for the expression. The `indent` parameter specifies the indentation level.
    fn debug(&self, indent: usize) -> String;

    /// Returns a Typist string for the expression.
    fn to_typist(&self) -> String;
}

// This allows for cloning a Box<dyn Expression>
dyn_clone::clone_trait_object!(Expression);

/// The `AsAny` trait provides a method for converting a type to `dyn Any`.
trait AsAny {
    /// Returns a reference to the type as a `dyn Any`, which can be downcast to its concrete type.
    fn as_any(&self) -> &dyn Any;
}

/// Implement `AsAny` for all types that implement `Expression`.
impl<T: Expression + 'static> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Implement `Debug` for `dyn Expression` to allow for printing expressions.
impl Debug for dyn Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.debug(0))
    }
}
