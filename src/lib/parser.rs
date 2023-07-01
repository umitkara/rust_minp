use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use crate::node::Node;
use crate::token::Token;
use crate::tokenizer::Tokenizer;

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    tokens: Vec<Token>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(expression: &'a str) -> Result<Self, ParseError> {
        let mut lexer = Tokenizer::new(expression);
        let mut tokens = Vec::new();
        let mut cur_token = Token::None;
        while cur_token != Token::EOF {
            let token = lexer.next().unwrap();
            cur_token = token.clone();
            tokens.push(token);
        }
        let cur_token = tokens.remove(0);
        Ok(Parser {
            tokenizer: lexer,
            current_token: cur_token,
            tokens,
        })
    }

    pub fn parse(&mut self) -> Result<Node, ParseError> {
        let result = self.expression_rule()?;
        if self.current_token != Token::EOF {
            return Err(ParseError::UnableToParse("Unable to parse".into()));
        }
        Ok(result)
    }

    fn get_next_token(&mut self) -> Result<(), ParseError> {
        if self.tokens.len() > 0 {
            self.current_token = self.tokens.remove(0);
            Ok(())
        } else {
            Err(ParseError::UnableToParse("Unable to parse".into()))
        }
    }

    fn expression_rule(&mut self) -> Result<Node, ParseError> {
        let mut left_result = self.term_rule()?;
        while self.current_token == Token::Plus || self.current_token == Token::Minus {
            match self.current_token {
                Token::Plus => {
                    self.get_next_token()?;
                    let right_result = self.term_rule()?;
                    left_result = Node::Addition(Box::new(left_result), Box::new(right_result))
                }
                Token::Minus => {
                    self.get_next_token()?;
                    let right_result = self.term_rule()?;
                    left_result = Node::Subtraction(Box::new(left_result), Box::new(right_result))
                }
                _ => return Ok(left_result)
            }
        }
        Ok(left_result)
    }

    fn term_rule(&mut self) -> Result<Node, ParseError> {
        let mut left_result = self.factor_rule()?;
        while self.current_token == Token::Asterisk || self.current_token == Token::Slash || self.current_token == Token::Caret || self.current_token == Token::Percent {
            match self.current_token {
                Token::Caret => {
                    self.get_next_token()?;
                    let right_result = self.factor_rule()?;
                    left_result = Node::Power(Box::new(left_result), Box::new(right_result))
                }
                Token::Asterisk => {
                    self.get_next_token()?;
                    let right_result = self.factor_rule()?;
                    left_result = Node::Multiplication(Box::new(left_result), Box::new(right_result))
                }
                Token::Slash => {
                    self.get_next_token()?;
                    let right_result = self.factor_rule()?;
                    left_result = Node::Division(Box::new(left_result), Box::new(right_result))
                }
                Token::Percent => {
                    self.get_next_token()?;
                    let right_result = self.factor_rule()?;
                    left_result = Node::Modulo(Box::new(left_result), Box::new(right_result))
                }
                _ => return Ok(left_result)
            }
        }

        Ok(left_result)
    }

    fn factor_rule(&mut self) -> Result<Node, ParseError> {
        match self.current_token {
            Token::LParen => {
                self.get_next_token()?;
                let result = self.expression_rule()?;
                if self.current_token != Token::RParen {
                    return Err(ParseError::InvalidOperator(format!("Invalid operator {:?}", self.current_token)));
                }
                self.get_next_token()?;
                Ok(result)
            }
            Token::RParen => {
                self.get_next_token()?;
                let result = self.factor_rule()?;
                Ok(result)
            }
            Token::Number(i) => {
                self.get_next_token()?;
                Ok(Node::Number(i))
            }
            Token::Plus => {
                self.get_next_token()?;
                let result = self.factor_rule()?;
                Ok(Node::Positive(Box::new(result)))
            }
            Token::Minus => {
                self.get_next_token()?;
                let result = self.factor_rule()?;
                Ok(Node::Negative(Box::new(result)))
            }
            _ => Err(ParseError::InvalidOperator(format!("Invalid operator {:?}", self.current_token)))
        }
    }
}

pub enum ParseError {
    UnableToParse(String),
    InvalidOperator(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            ParseError::UnableToParse(e) => write!(f, "Error in evaluating {}", e),
            ParseError::InvalidOperator(e) => write!(f, "Error in evaluating {}", e),
        }
    }
}

impl From<Box<dyn Error>> for ParseError {
    fn from(_value: Box<dyn Error>) -> Self {
        return ParseError::UnableToParse("Unable to parse".into());
    }
}