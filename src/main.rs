use crate::algebra::parser::parse_expression;

mod algebra;

fn main() {
    let input = "3+1*2*3*4+5*x";
    let (_, parsed) = parse_expression(input).unwrap();
    println!("Starting expression:");
    parsed.debug(0);

    let simplified_expression = parsed.simplify();
    println!("Simplified expression:");
    simplified_expression.debug(0);
}
