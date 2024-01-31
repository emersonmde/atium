use std::any::Any;

use crate::algebra::expression::Expression;

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
}

impl Variable {
    pub fn _new(value: &str) -> Self {
        Self {
            name: value.to_string(),
        }
    }
}

impl Expression for Variable {
    fn eval(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }

    fn simplify(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
