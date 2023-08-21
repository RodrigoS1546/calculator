use std::fmt::Display;
use std::vec::IntoIter;

use crate::tokenizer::Token;
use rust_decimal_macros::dec;

#[derive(Debug)]
pub struct ParseTree {
    pub token: Token,
    pub left: Option<Box<ParseTree>>,
    pub right: Option<Box<ParseTree>>,
}

impl ParseTree {
    fn new(token: Token) -> Self {
        Self {
            token,
            left: None,
            right: None,
        }
    }
}

#[derive(Debug)]
pub enum ParsingError {
    InvalidParenthesis,
    ExpectedExpression,
    ExpectedOperation,
    Unknown,
}

impl Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidParenthesis => write!(f, "Invalid Parenthesis"),
            Self::ExpectedExpression => write!(f, "Expected Expression"),
            Self::ExpectedOperation => write!(f, "Expected Operation"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Debug)]
enum Expression {
    Token(Token),
    Tree(ParseTree),
}

impl Expression {
    fn unwrap_token(self) -> Token {
        match self {
            Self::Token(x) => x,
            _ => panic!("Attempted to unwrap expression that was a tree."),
        }
    }
}

impl Default for Expression {
    fn default() -> Self {
        Self::Token(Token::default())
    }
}

trait NextExpression {
    fn next_expression(self: &mut Self) -> Result<Option<Expression>, ParsingError>;
}

impl NextExpression for IntoIter<Expression> {
    fn next_expression(&mut self) -> Result<Option<Expression>, ParsingError> {
        let expr = match self.next() {
            Some(expr) => expr,
            None => return Ok(None),
        };

        match expr {
            Expression::Token(Token::OpenParenthesis) => {
                let mut expression = Vec::new();
                let mut open_count = 0usize;
                let mut close_count = 0usize;
                while let Some(next) = self.next() {
                    match next {
                        Expression::Token(Token::OpenParenthesis) => {
                            open_count += 1;
                            expression.push(next);
                        }
                        Expression::Token(Token::CloseParenthesis) => {
                            close_count += 1;
                            if close_count > open_count {
                                break;
                            }
                            expression.push(next);
                        }
                        _ => expression.push(next),
                    }
                }
                let sub_tree = parse_expressions(expression)?;
                return Ok(Some(Expression::Tree(sub_tree)));
            }
            Expression::Token(Token::CloseParenthesis) => {
                return Err(ParsingError::InvalidParenthesis)
            }
            Expression::Token(Token::Invalid) => {
                panic!("An invalid token should never get to parsing stage.")
            }
            _ => return Ok(Some(expr)),
        }
    }
}

macro_rules! parse_operation {
    ($expressions:ident, $buffer:ident, $op:ident) => {
        while let Some(expr) = $expressions.next_expression()? {
            match expr {
                Expression::Token(Token::$op) => {
                    let last = match $buffer.pop() {
                        None
                        | Some(Expression::Token(
                            Token::Add | Token::Sub | Token::Mul | Token::Div,
                        )) => {
                            return Err(ParsingError::ExpectedExpression);
                        }
                        Some(expr) => expr,
                    };

                    let next = match $expressions.next_expression()? {
                        None
                        | Some(Expression::Token(
                            Token::Add | Token::Sub | Token::Mul | Token::Div,
                        )) => {
                            return Err(ParsingError::ExpectedExpression);
                        }
                        Some(expr) => expr,
                    };
                    $buffer.push(Expression::Tree(ParseTree {
                        token: Token::$op,
                        left: match last {
                            Expression::Token(lit) => Some(Box::new(ParseTree::new(lit))),
                            Expression::Tree(tree) => Some(Box::new(tree)),
                        },
                        right: match next {
                            Expression::Token(lit) => Some(Box::new(ParseTree::new(lit))),
                            Expression::Tree(tree) => Some(Box::new(tree)),
                        },
                    }));
                    $buffer.extend($expressions);
                    return parse_expressions($buffer);
                }
                _ => $buffer.push(expr),
            }
        }
    };
    ($expressions:ident, $buffer:ident, $op:ident, $op2:ident) => {
        while let Some(expr) = $expressions.next_expression()? {
            match expr {
                Expression::Token(Token::$op | Token::$op2) => {
                    let operation = expr.unwrap_token();
                    let last = if let Token::Add | Token::Sub = operation {
                        match $buffer.pop() {
                            Some(Expression::Token(
                                Token::Add | Token::Sub | Token::Mul | Token::Div,
                            )) => {
                                return Err(ParsingError::ExpectedExpression);
                            }
                            Some(expr) => expr,
                            None => Expression::Token(Token::Literal(dec!(0))),
                        }
                    } else {
                        match $buffer.pop() {
                            None
                            | Some(Expression::Token(
                                Token::Add | Token::Sub | Token::Mul | Token::Div,
                            )) => {
                                return Err(ParsingError::ExpectedExpression);
                            }
                            Some(expr) => expr,
                        }
                    };

                    let next = match $expressions.next_expression()? {
                        None
                        | Some(Expression::Token(
                            Token::Add | Token::Sub | Token::Mul | Token::Div,
                        )) => {
                            return Err(ParsingError::ExpectedExpression);
                        }
                        Some(expr) => expr,
                    };
                    $buffer.push(Expression::Tree(ParseTree {
                        token: operation,
                        left: match last {
                            Expression::Token(lit) => Some(Box::new(ParseTree::new(lit))),
                            Expression::Tree(tree) => Some(Box::new(tree)),
                        },
                        right: match next {
                            Expression::Token(lit) => Some(Box::new(ParseTree::new(lit))),
                            Expression::Tree(tree) => Some(Box::new(tree)),
                        },
                    }));
                    $buffer.extend($expressions);
                    return parse_expressions($buffer);
                }
                _ => $buffer.push(expr),
            }
        }
    };
}

fn parse_expressions(expressions: Vec<Expression>) -> Result<ParseTree, ParsingError> {
    if expressions.len() == 1 {
        match expressions.into_iter().next().unwrap_or_default() {
            Expression::Token(Token::Literal(x)) => return Ok(ParseTree::new(Token::Literal(x))),
            Expression::Token(Token::Ans) => return Ok(ParseTree::new(Token::Ans)),
            Expression::Token(Token::PI) => return Ok(ParseTree::new(Token::PI)),
            Expression::Tree(tree) => return Ok(tree),
            _ => return Err(ParsingError::Unknown),
        }
    }

    let mut expressions = expressions.into_iter();

    let mut buffer = Vec::new();

    parse_operation!(expressions, buffer, Exp);

    expressions = buffer.into_iter();
    buffer = Vec::new();

    parse_operation!(expressions, buffer, Mul, Div);

    expressions = buffer.into_iter();
    buffer = Vec::new();

    parse_operation!(expressions, buffer, Add, Sub);

    Err(ParsingError::ExpectedOperation)
}

pub fn parse(tokens: Vec<Token>) -> Result<ParseTree, ParsingError> {
    parse_expressions(
        tokens
            .into_iter()
            .map(|token| Expression::Token(token))
            .collect(),
    )
}
