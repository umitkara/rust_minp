use std::error;
use crate::node::Node;

pub fn eval(expr: Node) -> Result<f64, Box<dyn error::Error>> {
    use self::Node::*;
    match expr {
        Number(i) => Ok(i),
        Addition(expr1, expr2) => Ok(eval(*expr1)? + eval(*expr2)?),
        Subtraction(expr1, expr2) => Ok(eval(*expr1)? - eval(*expr2)?),
        Multiplication(expr1, expr2) => Ok(eval(*expr1)? * eval(*expr2)?),
        Division(expr1, expr2) => Ok(eval(*expr1)? / eval(*expr2)?),
        Positive(expr1) => Ok(eval(*expr1)?),
        Negative(expr1) => Ok(-(eval(*expr1)?)),
        Power(expr1, expr2) => Ok(eval(*expr1)?.powf(eval(*expr2)?)),
        Modulo(expr1, expr2) => Ok(eval(*expr1)? % eval(*expr2)?),
    }
}