use std::any::Any;
use std::fmt::Debug;

use crate::algebra::constant::Constant;
use crate::algebra::expression::Expression;

#[derive(Debug)]
pub struct Add {
    pub ops: Vec<Box<dyn Expression>>,
}

impl Add {
    pub fn new(ops: Vec<Box<dyn Expression>>) -> Self {
        Self { ops }
    }

    // TODO: Add flatten to Expression trait
    fn flatten(&self) -> Vec<Box<dyn Expression>> {
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
    fn eval(&self) -> Box<dyn Expression> {
        todo!()
    }

    fn simplify(&self) -> Box<dyn Expression> {
        println!("Starting ops {:?}\n", self.ops);
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
        // TODO: handle mixed expression (some constants, some variables)
        if ops
            .iter()
            .all(|op| op.as_any().downcast_ref::<Constant>().is_some())
        {
            let mut sum = 0.0;
            for op in ops.iter() {
                if let Some(op) = op.as_any().downcast_ref::<Constant>() {
                    sum += op.value;
                }
            }
            println!("Final sum: {}\n", sum);
            return Box::new(Constant::new(sum));
        }

        // Group like terms
        // Combine constants
        // Combine like variables
        // Handle additive inverses
        // Sort and reorganize the terms for readability (optional)
        // Check for simplification to a single term
        // Construct and return the simplified Add expression

        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
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
        if let Some(constant) = simplified.as_any().downcast_ref::<Constant>() {
            assert_eq!(constant.value, 3.0);
        } else {
            panic!("Expected Constant");
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
        if let Some(constant) = simplified.as_any().downcast_ref::<Constant>() {
            assert_eq!(constant.value, 6.0);
        } else {
            panic!("Expected Constant");
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
        if let Some(constant) = simplified.as_any().downcast_ref::<Constant>() {
            assert_eq!(constant.value, 6.0);
        } else {
            panic!("Expected Constant");
        }
    }
}
