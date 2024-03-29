use std::any::Any;

use crate::algebra::constant::Constant;
use crate::algebra::expression::Expression;
use crate::algebra::multiply::Multiply;

/// `Add` struct represents an addition operation in an expression tree.
/// It contains a vector of `Expression` trait objects.
pub struct Add {
    pub ops: Vec<Box<dyn Expression>>,
}

impl Add {
    pub fn new(ops: Vec<Box<dyn Expression>>) -> Self {
        Self { ops }
    }

    /// Flattens nested `Add` expressions into a single-level `Add` expression.
    /// This is a helper method used in the `simplify` method.
    fn flatten(&self) -> Vec<Box<dyn Expression>> {
        // TODO: Add flatten to Expression trait
        let mut flattened_ops = Vec::new();
        for op in &self.ops {
            if let Some(mul) = op.as_any().downcast_ref::<Add>() {
                let more_flattened_ops = mul.flatten();
                flattened_ops.extend(more_flattened_ops);
            } else {
                flattened_ops.push(op.clone());
            }
        }
        flattened_ops
    }
}

impl Expression for Add {
    /// Evaluates the expression and returns a new expression.
    fn eval(&self) -> Box<dyn Expression> {
        todo!()
    }

    /// Simplifies the expression and returns a new simplified expression.
    /// This method implements several algebraic simplification rules, such as eliminating addition by 0,
    /// evaluating constant addition, and others.
    fn simplify(&self) -> Box<dyn Expression> {
        // Flatten nested Add expressions
        let flattened_ops = self.flatten();

        // Eliminate zero terms and simplify all operands
        let ops: Vec<Box<dyn Expression>> = flattened_ops
            .iter()
            .map(|op| op.simplify())
            .filter(|op| {
                if let Some(op) = op.as_any().downcast_ref::<Constant>() {
                    op.value != 0.0
                } else {
                    true
                }
            })
            .collect();

        // Sum all constants
        let (constants, mut ops): (Vec<_>, Vec<_>) = ops
            .into_iter()
            .partition(|op| op.as_any().downcast_ref::<Constant>().is_some());
        let sum: f64 = constants
            .iter()
            .map(|op| op.as_any().downcast_ref::<Constant>().unwrap().value)
            .sum();

        ops.push(Box::<Constant>::new(Constant::new(sum)));

        // Group like terms
        // Combine constants
        // Combine like variables
        // Handle additive inverses
        // Sort and reorganize the terms for readability (optional)
        // Check for simplification to a single term
        // Construct and return the simplified Add expression

        Box::new(Self { ops })
    }

    /// Returns a reference to the expression as a `dyn Any`, which can be downcast to its concrete type.
    fn as_any(&self) -> &dyn Any {
        self
    }

    /// Returns a debug string for the expression. The `indent` parameter specifies the indentation level.
    fn debug(&self, indent: usize) -> String {
        let mut output = format!("{}Add {{\n", " ".repeat(indent));
        for op in &self.ops {
            output.push_str(&op.debug(indent + 2));
        }
        output.push_str(&format!("{}}}\n", " ".repeat(indent)));
        output
    }

    /// Returns a Typist string for the expression.
    fn to_typist(&self) -> String {
        let mut parts: Vec<String> = Vec::new();
        for op in &self.ops {
            let part = op.to_typist();
            // Nested expressions might need parentheses, but simple constants or variables do not.
            if op.as_any().downcast_ref::<Multiply>().is_some()
                || op.as_any().downcast_ref::<Add>().is_some()
            {
                parts.push(format!("({})", part));
            } else {
                parts.push(part);
            }
        }
        parts.join(" + ")
    }
}

impl Clone for Add {
    fn clone(&self) -> Self {
        Add {
            ops: self.ops.to_vec(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::algebra::constant::Constant;

    use super::*;

    #[test]
    fn add_simplify_with_zero() {
        let add = Add::new(vec![
            Box::new(Constant::new(1.0)),
            Box::new(Constant::new(0.0)),
            Box::new(Constant::new(2.0)),
        ]);
        let simplified = add.simplify();
        if let Some(simplified_add) = simplified.as_any().downcast_ref::<Add>() {
            assert_eq!(
                simplified_add
                    .ops
                    .first()
                    .unwrap()
                    .as_any()
                    .downcast_ref::<Constant>()
                    .unwrap()
                    .value,
                3.0
            );
        } else {
            panic!("Expected Constant, found {:?}", simplified);
        }
    }

    #[test]
    fn add_simplify_with_no_zero() {
        let add = Add::new(vec![
            Box::new(Constant::new(1.0)),
            Box::new(Constant::new(2.0)),
            Box::new(Constant::new(3.0)),
        ]);
        let simplified = add.simplify();
        if let Some(simplified_add) = simplified.as_any().downcast_ref::<Add>() {
            assert_eq!(
                simplified_add
                    .ops
                    .first()
                    .unwrap()
                    .as_any()
                    .downcast_ref::<Constant>()
                    .unwrap()
                    .value,
                6.0
            )
        } else {
            panic!("Expected Constant, found {:?}", simplified);
        }
    }

    #[test]
    fn add_simplify_with_nested_add() {
        let nested_add = Add::new(vec![
            Box::new(Constant::new(1.0)),
            Box::new(Constant::new(2.0)),
        ]);
        let add = Add::new(vec![Box::new(Constant::new(3.0)), Box::new(nested_add)]);
        let simplified = add.simplify();
        if let Some(simplified_add) = simplified.as_any().downcast_ref::<Add>() {
            assert_eq!(
                simplified_add
                    .ops
                    .first()
                    .unwrap()
                    .as_any()
                    .downcast_ref::<Constant>()
                    .unwrap()
                    .value,
                6.0
            )
        } else {
            panic!("Expected Constant, found {:?}", simplified);
        }
    }

    #[test]
    fn add_simplify_with_negative_constant() {
        let add = Add::new(vec![
            Box::new(Constant::new(5.0)),
            Box::new(Constant::new(-3.0)),
        ]);
        let simplified = add.simplify();
        assert_eq!(simplified.to_typist(), "2");
    }
}
