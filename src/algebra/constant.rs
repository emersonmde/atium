use std::any::Any;

use crate::algebra::expression::Expression;

/// `Constant` struct represents a constant value in an expression tree.
/// It contains a `value` field which is a `f64`.
#[derive(Debug, Clone)]
pub struct Constant {
    pub value: f64,
}

impl Constant {
    /// Constructs a new `Constant` instance.
    ///
    /// # Arguments
    ///
    /// * `value` - A floating point number that represents the value of the constant.
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

impl Expression for Constant {
    /// Evaluates the expression and returns a new expression.
    /// For a `Constant`, it returns a clone of itself.
    fn eval(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }

    /// Simplifies the expression and returns a new simplified expression.
    /// For a `Constant`, it returns a clone of itself.
    fn simplify(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }

    /// Returns a reference to the expression as a `dyn Any`, which can be downcast to its concrete type.
    fn as_any(&self) -> &dyn Any {
        self
    }

    /// Returns a debug string for the expression. The `indent` parameter specifies the indentation level.
    fn debug(&self, indent: usize) -> String {
        format!(
            "{}Constant {{ value: {} }}\n",
            " ".repeat(indent),
            self.value
        )
    }

    /// Returns a Typist string for the expression.
    /// For a `Constant`, it directly returns the string representation of its value.
    fn to_typist(&self) -> String {
        self.value.to_string()
    }
}
