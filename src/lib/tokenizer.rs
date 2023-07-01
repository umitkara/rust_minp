use std::str::Chars;
use std::iter::Peekable;
use crate::token::Token;

pub struct Tokenizer<'a> {
    expression: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(new_expression: &'a str) -> Self {
        Tokenizer {
            expression: new_expression.chars().peekable()
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let next_char = self.expression.next();

        match next_char {
            Some('0'..='9') => {
                let mut number = next_char?.to_string();
                while let Some(next_char) = self.expression.peek() {
                    if next_char.is_numeric() || next_char == &'.' {
                        number.push(self.expression.next()?);
                    } else if next_char == &'(' {
                        return None;
                    } else {
                        break;
                    }
                }
                Some(Token::Number(number.parse::<f64>().unwrap()))
            }
            Some('+') => Some(Token::Plus),
            Some('-') => Some(Token::Minus),
            Some('*') => Some(Token::Asterisk),
            Some('/') => Some(Token::Slash),
            Some('^') => Some(Token::Caret),
            Some('%') => Some(Token::Percent),
            Some('(') => Some(Token::LParen),
            Some(')') => Some(Token::RParen),
            None => Some(Token::EOF),
            Some(_) => Some(Token::None),
        }
    }
}