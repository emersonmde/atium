use crate::algebra::add::Add;
use std::any::Any;

use crate::algebra::constant::Constant;
use crate::algebra::expression::Expression;

/// `Multiply` struct represents a multiplication operation in an expression tree.
/// It contains a vector of `Expression` trait objects, which can be any type that implements the `Expression` trait.
pub struct Multiply {
    pub ops: Vec<Box<dyn Expression>>,
}

impl Multiply {
    /// Constructs a new `Multiply` instance.
    pub fn new(ops: Vec<Box<dyn Expression>>) -> Self {
        Self { ops }
    }

    /// Flattens nested `Multiply` expressions into a single-level `Multiply` expression.
    /// This is a helper method used in the `simplify` method.
    fn flatten(&self) -> Vec<Box<dyn Expression>> {
        let mut flattened_ops = Vec::new();
        for op in &self.ops {
            if let Some(mul) = op.as_any().downcast_ref::<Multiply>() {
                let more_flattened_ops = mul.flatten();
                flattened_ops.extend(more_flattened_ops);
            } else {
                flattened_ops.push(op.clone());
            }
        }
        flattened_ops
    }
}

impl Expression for Multiply {
    fn eval(&self) -> Box<dyn Expression> {
        // Simplify
        // Eval all children
        // Multiply all children
        todo!("Implement eval for Multiply")
    }

    /// Simplifies the expression and returns a new simplified expression.
    /// This method implements several algebraic simplification rules, such as eliminating multiplication by 1,
    /// evaluating constant multiplication, and others.
    fn simplify(&self) -> Box<dyn Expression> {
        // flatten nested multiply expressions
        let flattened_ops = self.flatten();

        // Handle 0
        if flattened_ops.iter().any(|op| {
            if let Some(op) = op.as_any().downcast_ref::<Constant>() {
                op.value == 0.0
            } else {
                false
            }
        }) {
            return Box::new(Constant::new(0.0));
        }

        // Filter out multiplying by 1 and simplify all operands
        let ops: Vec<Box<dyn Expression>> = flattened_ops
            .iter()
            .map(|op| op.simplify())
            .filter(|op| {
                if let Some(op) = op.as_any().downcast_ref::<Constant>() {
                    op.value != 1.0
                } else {
                    true
                }
            })
            .collect();

        // TODO: create identity trait and implement to combine like terms and calculate product of constants
        // Combine Like Terms
        // Evaluate Constant Multiplication
        if ops
            .iter()
            .all(|op| op.as_any().downcast_ref::<Constant>().is_some())
        {
            let mut product = 1.0;
            for op in ops.iter() {
                if let Some(op) = op.as_any().downcast_ref::<Constant>() {
                    product *= op.value;
                }
            }
            return Box::new(Constant::new(product));
        }

        // Multiplication of Inverses
        // Sort and Group Operands
        // Distribute Multiplication over Addition
        // Simplify Multiplication with Variables and Coefficients
        // Eliminate Unit Coefficients
        // Use Algebraic Identities
        // Simplify Products Involving Exponents
        // Consider Special Cases and Simplifications
        // Simplify and Reduce Expression
        // Return Simplified Expression

        Box::new(Self { ops })
    }

    /// Returns a reference to the expression as a `dyn Any`, which can be downcast to its concrete type.
    fn as_any(&self) -> &dyn Any {
        self
    }

    /// Returns a debug string for the expression. The `indent` parameter specifies the indentation level.
    fn debug(&self, indent: usize) -> String {
        let mut output = format!("{}Multiply {{\n", " ".repeat(indent));
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
            // Use parentheses for nested expressions for clarity
            let part = op.to_typist();
            if op.as_any().downcast_ref::<Multiply>().is_some()
                || op.as_any().downcast_ref::<Add>().is_some()
            {
                parts.push(format!("({})", part));
            } else {
                parts.push(part);
            }
        }
        parts.join(" ") // Join with multiplication symbol; adjust based on Typist conventions if necessary
    }
}

impl Clone for Multiply {
    fn clone(&self) -> Self {
        Multiply {
            ops: self.ops.to_vec(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::algebra::add::Add;
    use crate::algebra::constant::Constant;

    use super::*;

    #[test]
    fn multiply_simplify_with_zero_and_one() {
        let multiply = Multiply::new(vec![
            Box::new(Constant::new(1.0)),
            Box::new(Constant::new(0.0)),
            Box::new(Constant::new(2.0)),
        ]);
        let simplified = multiply.simplify();
        if let Some(constant) = simplified.as_any().downcast_ref::<Constant>() {
            assert_eq!(constant.value, 0.0);
        } else {
            panic!("Expected Constant");
        }
    }

    #[test]
    fn multiply_simplify_with_no_zero_or_one() {
        let multiply = Multiply::new(vec![
            Box::new(Constant::new(2.0)),
            Box::new(Constant::new(3.0)),
            Box::new(Constant::new(4.0)),
        ]);
        let simplified = multiply.simplify();
        if let Some(constant) = simplified.as_any().downcast_ref::<Constant>() {
            assert_eq!(constant.value, 24.0);
        } else {
            panic!("Expected Constant");
        }
    }

    #[test]
    fn multiply_simplify_with_nested_multiply() {
        let nested_multiply = Multiply::new(vec![
            Box::new(Constant::new(2.0)),
            Box::new(Constant::new(3.0)),
        ]);
        let multiply = Multiply::new(vec![
            Box::new(Constant::new(4.0)),
            Box::new(nested_multiply),
        ]);
        let simplified = multiply.simplify();
        if let Some(constant) = simplified.as_any().downcast_ref::<Constant>() {
            assert_eq!(constant.value, 24.0);
        } else {
            panic!("Expected Constant");
        }
    }

    #[test]
    fn multiply_simplify_with_nested_add() {
        let nested_add = Add::new(vec![
            Box::new(Constant::new(2.0)),
            Box::new(Constant::new(3.0)),
        ]);
        let multiply = Multiply::new(vec![Box::new(Constant::new(4.0)), Box::new(nested_add)]);
        let simplified = multiply.simplify();
        if let Some(constant) = simplified.as_any().downcast_ref::<Constant>() {
            assert_eq!(constant.value, 20.0);
        } else {
            panic!("Expected Constant");
        }
    }
}
