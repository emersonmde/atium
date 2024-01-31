use crate::algebra::parser::parse_expression;

mod algebra;

fn main() {
    let input = "3+1*2*3*4+5";
    let (_, parsed) = parse_expression(input).unwrap();
    println!("Starting expression: {:?}", parsed);
    println!("{:?}", parsed.simplify());
}
