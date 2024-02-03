use crate::algebra::parser::parse_expression;

mod algebra;

fn main() {
    let input = "3+1*2*3*4+5*x";
    // let input = "2*3*4*4*5";
    let (_, parsed) = parse_expression(input).unwrap();
    println!("Starting expression: {:?}", parsed);

    let simplified_expression = parsed.simplify();
    println!("Simplified expression: {:?}", simplified_expression);
}
