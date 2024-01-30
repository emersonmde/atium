use crate::algebra::constant::Constant;
use crate::algebra::expression::Expression;
use crate::algebra::multiply::Multiply;
use crate::algebra::variable::Variable;

mod algebra;


fn main() {
    let nested_args: Vec<Box<dyn Expression>> = vec![
        Box::new(Constant::new(7.0)),
        // Box::new(Constant::new(0.0)),
        Box::new(Constant::new(5.0)),
    ];
    let args: Vec<Box<dyn Expression>> = vec![
        Box::new(Constant::new(3.0)),
        Box::new(Constant::new(1.0)),
        Box::new(Constant::new(1.0)),
        Box::new(Variable::new("x")),
        Box::new(Constant::new(3.0)),
        // Box::new(Constant::new(0.0)),
        Box::new(Constant::new(2.0)),
        Box::new(Multiply::new(nested_args)),
    ];
    let mul = Multiply::new(args);
    println!("\n\n");
    println!("Result {:?}", mul.simplify());
}
