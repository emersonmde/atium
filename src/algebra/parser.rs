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

fn parse_number(input: &str) -> IResult<&str, Box<dyn Expression>> {
    map_res(digit1, |digit_str: &str| {
        digit_str
            .parse::<f64>()
            .map(|num| Box::new(Constant::new(num)) as Box<dyn Expression>)
    })(input)
}

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

fn parse_term(input: &str) -> IResult<&str, Box<dyn Expression>> {
    let (input, init) = parse_factor(input)?;
    let (input, ops) = many0(preceded(tag("*"), parse_factor))(input)?;
    Ok((
        input,
        ops.into_iter()
            .fold(init, |acc, val| Box::new(Multiply::new(vec![acc, val]))),
    ))
}

pub fn parse_expression(input: &str) -> IResult<&str, Box<dyn Expression>> {
    let (input, init) = parse_term(input)?;
    let (input, ops) = many0(preceded(tag("+"), parse_term))(input)?;
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
}
