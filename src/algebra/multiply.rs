use std::any::Any;

use crate::algebra::constant::Constant;
use crate::algebra::expression::Expression;

#[derive(Debug)]
pub struct Multiply {
    pub ops: Vec<Box<dyn Expression>>,
}

impl Multiply {
    pub fn new(ops: Vec<Box<dyn Expression>>) -> Self {
        Self {
            ops,
        }
    }

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
        (*self).simplify()
    }


    fn simplify(&self) -> Box<dyn Expression> {
        println!("Starting ops {:?}\n", self.ops);
        // Flatten Nested Multiplications: If operands are multiplication expressions, flatten them into a single multiplication operation to reveal further simplification opportunities.
        // Multiplication by One: Remove any operands that are one, since they do not change the product.
        let flattened_ops = self.flatten();
        println!("Flattened ops {:?}\n", flattened_ops);
        let ops: Vec<Box<dyn Expression>> = flattened_ops.iter()
            .map(|op| op.simplify())
            .filter(|op| {
                if let Some(op) = op.as_any().downcast_ref::<Constant>() {
                    op.value != 1.0
                } else {
                    true
                }
            })
            .collect();

        println!("Filtered and flattened ops {:?}\n", ops);

        // Zero Multiplication: If any operand is zero, the entire expression simplifies to zero, as anything multiplied by zero is zero.
        if ops.iter().any(|op| {
            if let Some(op) = op.as_any().downcast_ref::<Constant>() {
                op.value == 0.0
            } else {
                false
            }
        }) {
            return Box::new(Constant::new(0.0));
        }


        // TODO: create identity trait and implement to combine like terms and calculate product of constants
        // Combine Like Terms: For operands with variables, combine like terms by adding their exponents, e.g., x^a * x^b becomes x^(a+b).
        // Evaluate Constant Multiplication: Calculate the product of all constant operands. If all are constants, return the product as a new constant expression.
        if ops.iter().all(|op| op.as_any().downcast_ref::<Constant>().is_some()) {
            let mut product = 1.0;
            for op in self.ops.iter() {
                if let Some(op) = op.as_any().downcast_ref::<Constant>() {
                    product *= op.value;
                }
            }
            return Box::new(Constant::new(product));
        }

        // Multiplication of Inverses: Simplify expressions where operands are inverses of each other, e.g., x * (1/x) simplifies to 1.
        // Sort and Group Operands: Sort operands for readability and group like terms to facilitate combining them.
        // Distribute Multiplication over Addition: If applicable, apply distributive properties, though this may lead to expansion rather than simplification.
        // Simplify Multiplication with Variables and Coefficients: Group and multiply coefficients separately from variables for simplification.
        // Eliminate Unit Coefficients: After multiplication, if a term's coefficient is one, omit it for simplicity.
        // Use Algebraic Identities: Apply algebraic identities where possible to simplify the expression.
        // Simplify Products Involving Exponents: Apply rules for products of powers, such as x^a * x^b = x^(a+b).
        // Consider Special Cases and Simplifications: Look for simplifications based on special cases, properties of the operands, or known identities.
        // Simplify and Reduce Expression: After applying all the above steps, if the expression can be further simplified or reduced, do so.
        // Return Simplified Expression: Ultimately, return the most simplified version of the multiplication expression.

        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Clone for Multiply {
    fn clone(&self) -> Self {
        Multiply {
            ops: self.ops.iter().map(|op| op.clone()).collect(),
        }
    }
}