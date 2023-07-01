#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Number(f64),
    Addition(Box<Node>, Box<Node>),
    Subtraction(Box<Node>, Box<Node>),
    Multiplication(Box<Node>, Box<Node>),
    Division(Box<Node>, Box<Node>),
    Power(Box<Node>, Box<Node>),
    Modulo(Box<Node>, Box<Node>),
    Positive(Box<Node>),
    Negative(Box<Node>),
}