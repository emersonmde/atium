use std::any::Any;

use crate::algebra::expression::Expression;

/// `Variable` struct represents a variable in an expression tree.
/// It contains a `name` field which is a `String`.
#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
}

impl Variable {
    /// Constructs a new `Variable` instance.
    ///
    /// # Arguments
    ///
    /// * `value` - A string slice that represents the name of the variable.
    pub fn new(value: &str) -> Self {
        Self {
            name: value.to_string(),
        }
    }
}

impl Expression for Variable {
    /// Evaluates the expression and returns a new expression.
    /// For a `Variable`, it returns a clone of itself.
    fn eval(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }

    /// Simplifies the expression and returns a new simplified expression.
    /// For a `Variable`, it returns a clone of itself.
    fn simplify(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }

    /// Returns a reference to the expression as a `dyn Any`, which can be downcast to its concrete type.
    fn as_any(&self) -> &dyn Any {
        self
    }

    /// Returns a debug string for the expression. The `indent` parameter specifies the indentation level.
    fn debug(&self, indent: usize) -> String {
        format!("{}Variable {{ name: {} }}\n", " ".repeat(indent), self.name)
    }

    /// Returns a Typist string for the expression.
    /// For a `Variable`, it directly returns the variable name.
    fn to_typist(&self) -> String {
        self.name.clone() // Directly return the variable name
    }
}
