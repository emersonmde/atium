use std::any::Any;

use crate::algebra::expression::Expression;

#[derive(Debug, Clone)]
pub struct Constant {
    pub value: f64,
}

impl Constant {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

impl Expression for Constant {
    fn eval(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }

    fn simplify(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn debug(&self, indent: usize) -> String {
        format!(
            "{}Constant {{ value: {} }}\n",
            " ".repeat(indent),
            self.value
        )
    }

    fn to_typist(&self) -> String {
        self.value.to_string()
    }
}
