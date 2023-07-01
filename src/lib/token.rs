#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub enum Token {
    // Ex: 1, 1.0
    Number(f64),
    // +
    Plus,
    // -
    Minus,
    // *
    Asterisk,
    // /
    Slash,
    // (
    LParen,
    // )
    RParen,
    // ^
    Caret,
    // %
    Percent,
    EOF,
    None,
}