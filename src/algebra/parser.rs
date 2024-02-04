use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::preceded;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::digit1,
    character::complete::multispace0, combinator::map_res, sequence::delimited, IResult,
};

use crate::algebra::add::Add;
use crate::algebra::constant::Constant;
use crate::algebra::expression::Expression;
use crate::algebra::multiply::Multiply;
use crate::algebra::variable::Variable;

/// Parses a variable from the input string.
///
/// # Arguments
///
/// * `input` - A string slice that should begin with a variable name.
///
/// # Returns
///
/// * `IResult<&str, Box<dyn Expression>>` - On success, the function returns the remaining input and the parsed variable as a `Box<dyn Expression>`.
fn parse_variable(input: &str) -> IResult<&str, Box<dyn Expression>> {
    map(
        delimited(
            multispace0,
            // TODO: use alpha1 instead?
            alt((tag("x"), tag("y"), tag("z"))),
            multispace0,
        ),
        |var_str: &str| Box::new(Variable::new(var_str)) as Box<dyn Expression>,
    )(input)
}

/// Parses a number from the input string.
///
/// # Arguments
///
/// * `input` - A string slice that should begin with a number.
///
/// # Returns
///
/// * `IResult<&str, Box<dyn Expression>>` - On success, the function returns the remaining input and the parsed number as a `Box<dyn Expression>`.
fn parse_number(input: &str) -> IResult<&str, Box<dyn Expression>> {
    map_res(digit1, |digit_str: &str| {
        digit_str
            .parse::<f64>()
            .map(|num| Box::new(Constant::new(num)) as Box<dyn Expression>)
    })(input)
}

/// Parses a factor from the input string.
///
/// A factor is either a number, a variable, or an expression in parentheses.
///
/// # Arguments
///
/// * `input` - A string slice that should begin with a factor.
///
/// # Returns
///
/// * `IResult<&str, Box<dyn Expression>>` - On success, the function returns the remaining input and the parsed factor as a `Box<dyn Expression>`.
fn parse_factor(input: &str) -> IResult<&str, Box<dyn Expression>> {
    delimited(
        multispace0,
        alt((
            delimited(tag("("), parse_expression, tag(")")),
            parse_variable,
            parse_number,
        )),
        multispace0,
    )(input)
}

/// Parses a term from the input string.
///
/// A term is a sequence of factors separated by multiplication operators.
///
/// # Arguments
///
/// * `input` - A string slice that should begin with a term.
///
/// # Returns
///
/// * `IResult<&str, Box<dyn Expression>>` - On success, the function returns the remaining input and the parsed term as a `Box<dyn Expression>`.
fn parse_term(input: &str) -> IResult<&str, Box<dyn Expression>> {
    let (input, init) = parse_factor(input)?;
    let (input, ops) = many0(preceded(tag("*"), parse_factor))(input)?;
    Ok((
        input,
        ops.into_iter()
            .fold(init, |acc, val| Box::new(Multiply::new(vec![acc, val]))),
    ))
}

/// Parses an expression from the input string.
///
/// An expression is a sequence of terms separated by addition or subtraction operators.
///
/// # Arguments
///
/// * `input` - A string slice that should begin with an expression.
///
/// # Returns
///
/// * `IResult<&str, Box<dyn Expression>>` - On success, the function returns the remaining input and the parsed expression as a `Box<dyn Expression>`.
pub fn parse_expression(input: &str) -> IResult<&str, Box<dyn Expression>> {
    let (input, init) = parse_term(input)?;
    let (input, ops) = many0(alt((
        preceded(tag("+"), parse_term),
        // Handle subtraction by negating the term following the '-'
        map(
            preceded(tag("-"), parse_term),
            |term: Box<dyn Expression>| {
                Box::new(Add::new(vec![
                    Box::new(Constant::new(0.0)),
                    Box::new(Multiply::new(vec![Box::new(Constant::new(-1.0)), term])),
                ])) as Box<dyn Expression>
            },
        ),
    )))(input)?;

    let result = ops
        .into_iter()
        .fold(init, |acc, val| Box::new(Add::new(vec![acc, val])));

    if input.is_empty() {
        Ok((input, result))
    } else {
        let err = nom::error::Error::new(input, nom::error::ErrorKind::Eof);
        Err(nom::Err::Failure(err))
    }
}

#[cfg(test)]
mod tests {
    use crate::algebra::add::Add;
    use crate::algebra::constant::Constant;
    use crate::algebra::multiply::Multiply;

    use super::*;

    #[test]
    fn parse_single_number() {
        let input = "3";
        let (_, parsed) = parse_expression(input).unwrap();
        if let Some(constant) = parsed.as_any().downcast_ref::<Constant>() {
            assert_eq!(constant.value, 3.0);
        } else {
            panic!("Expected Constant");
        }
    }

    #[test]
    fn parse_addition_expression() {
        let input = "3+2";
        let (_, parsed) = parse_expression(input).unwrap();
        if let Some(add) = parsed.as_any().downcast_ref::<Add>() {
            assert_eq!(add.ops.len(), 2);
        } else {
            panic!("Expected Add");
        }
    }

    #[test]
    fn parse_multiplication_expression() {
        let input = "3*2";
        let (_, parsed) = parse_expression(input).unwrap();
        if let Some(multiply) = parsed.as_any().downcast_ref::<Multiply>() {
            assert_eq!(multiply.ops.len(), 2);
        } else {
            panic!("Expected Multiply");
        }
    }

    #[test]
    fn parse_complex_expression() {
        let input = "3+2*4";
        let (_, parsed) = parse_expression(input).unwrap();
        if let Some(add) = parsed.as_any().downcast_ref::<Add>() {
            assert_eq!(add.ops.len(), 2);
            if let Some(multiply) = add.ops[1].as_any().downcast_ref::<Multiply>() {
                assert_eq!(multiply.ops.len(), 2);
            } else {
                panic!("Expected Multiply in Add");
            }
        } else {
            panic!("Expected Add");
        }
    }

    #[test]
    fn parse_invalid_expression() {
        let input = "3+*4";
        let result = parse_expression(input);
        assert!(result.is_err());
    }

    #[test]
    fn parse_variable_expression() {
        let input = "x";
        let (_, parsed) = parse_expression(input).unwrap();
        if let Some(variable) = parsed.as_any().downcast_ref::<Variable>() {
            assert_eq!(variable.name, "x");
        } else {
            panic!("Expected Variable");
        }
    }

    #[test]
    fn parse_subtraction_expression() {
        let input = "3-2";
        let (_, parsed) = parse_expression(input)
            .unwrap_or_else(|_| panic!("Failed to parse expression '{}'", input));

        // Expect the top-level operation to be an `Add`.
        if let Some(add) = parsed.as_any().downcast_ref::<Add>() {
            // The `Add` operation should have exactly two operands: `3` and the negation structure.
            assert_eq!(
                add.ops.len(),
                2,
                "Expected Add operation to have 2 operands, found {}",
                add.ops.len()
            );

            // The first operand should be the constant `3`.
            if let Some(constant) = add.ops[0].as_any().downcast_ref::<Constant>() {
                assert_eq!(
                    constant.value, 3.0,
                    "Expected first operand to be 3, found {}",
                    constant.value
                );
            } else {
                panic!(
                    "Expected first operand to be Constant(3), found {:?}",
                    add.ops[0]
                );
            }

            // The second operand should be an `Add` operation representing the negated term.
            if let Some(inner_add) = add.ops[1].as_any().downcast_ref::<Add>() {
                // This `Add` operation should have exactly two operands: `0` and the multiplication by `-1`.
                assert_eq!(
                    inner_add.ops.len(),
                    2,
                    "Expected inner Add operation to have 2 operands for negation, found {}",
                    inner_add.ops.len()
                );

                // The first operand of this inner `Add` should be the constant `0`.
                if let Some(constant) = inner_add.ops[0].as_any().downcast_ref::<Constant>() {
                    assert_eq!(
                        constant.value, 0.0,
                        "Expected first operand of inner Add to be 0, found {}",
                        constant.value
                    );
                } else {
                    panic!(
                        "Expected first operand of inner Add to be Constant(0), found {:?}",
                        inner_add.ops[0]
                    );
                }

                // The second operand should be a `Multiply` operation with `-1` and `2`.
                if let Some(multiply) = inner_add.ops[1].as_any().downcast_ref::<Multiply>() {
                    assert_eq!(
                        multiply.ops.len(),
                        2,
                        "Expected Multiply operation to have 2 operands for negation, found {}",
                        multiply.ops.len()
                    );

                    if let Some(constant) = multiply.ops[0].as_any().downcast_ref::<Constant>() {
                        assert_eq!(
                            constant.value, -1.0,
                            "Expected first operand of Multiply to be -1 for negation, found {}",
                            constant.value
                        );
                    } else {
                        panic!(
                            "Expected first operand of Multiply to be Constant(-1), found {:?}",
                            multiply.ops[0]
                        );
                    }

                    if let Some(constant) = multiply.ops[1].as_any().downcast_ref::<Constant>() {
                        assert_eq!(
                            constant.value, 2.0,
                            "Expected second operand of Multiply to be 2, found {}",
                            constant.value
                        );
                    } else {
                        panic!(
                            "Expected second operand of Multiply to be Constant(2), found {:?}",
                            multiply.ops[1]
                        );
                    }
                } else {
                    panic!(
                        "Expected second operand of inner Add to be Multiply(-1, 2), found {:?}",
                        inner_add.ops[1]
                    );
                }
            } else {
                panic!("Expected second operand of top-level Add to be an inner Add operation representing negation, found {:?}", add.ops[1]);
            }
        } else {
            panic!(
                "Expected parsed expression to be an Add operation, found {:?}",
                parsed
            );
        }
    }

    #[test]
    fn parse_expression_with_whitespace() {
        let input = " 3 + 2 * 4 ";
        let (_, parsed) = parse_expression(input).unwrap();
        if let Some(add) = parsed.as_any().downcast_ref::<Add>() {
            assert_eq!(add.ops.len(), 2);
            if let Some(multiply) = add.ops[1].as_any().downcast_ref::<Multiply>() {
                assert_eq!(multiply.ops.len(), 2);
            } else {
                panic!("Expected Multiply");
            }
        } else {
            panic!("Expected Add");
        }
    }
}
