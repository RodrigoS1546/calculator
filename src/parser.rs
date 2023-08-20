use std::vec::IntoIter;

use crate::tokenizer::Token;

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

#[derive(Debug)]
enum Expression {
    Token(Token),
    Tree(ParseTree),
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
                        },
                        Expression::Token(Token::CloseParenthesis) => {
                            close_count += 1;
                            if close_count > open_count {
                                break;
                            }
                            expression.push(next);
                        },
                        _ => expression.push(next),
                    }
                }
                let sub_tree = parse_expressions(expression)?;
                return Ok(Some(Expression::Tree(sub_tree)));
            },
            Expression::Token(Token::CloseParenthesis) => return Err(ParsingError::InvalidParenthesis),
            Expression::Token(Token::Invalid) => panic!("An invalid token should never get to parsing stage."),
            _ => return Ok(Some(expr)),
        }
    }
}

macro_rules! parse_operator {
    ($expressions:ident, $buffer:ident, $op:ident) => {
        while let Some(expr) = $expressions.next_expression()? {
            match expr {
                Expression::Token(Token::$op) => {
                    let last = match $buffer.pop() {
                        None | Some(Expression::Token(Token::Add | Token::Sub | Token::Mul | Token::Div)) => {
                            return Err(ParsingError::ExpectedExpression);
                        }
                        Some(expr) => expr,
                    };
            
                    let next = match $expressions.next_expression()? {
                        None | Some(Expression::Token(Token::Add | Token::Sub | Token::Mul | Token::Div)) => {
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
                },
                _ => $buffer.push(expr),
            }
        }
    };
}

fn parse_expressions(expressions: Vec<Expression>) -> Result<ParseTree, ParsingError> {
    if expressions.len() == 1 {
        match expressions.into_iter().next().unwrap_or_default() {
            Expression::Token(Token::Literal(x)) => return Ok(ParseTree::new(Token::Literal(x))),
            Expression::Tree(tree) => return Ok(tree),
            _ => return Err(ParsingError::Unknown),
        }
    }

    let mut expressions = expressions.into_iter();

    let mut buffer = Vec::new();

    parse_operator!(expressions, buffer, Exp);

    expressions = buffer.into_iter();
    buffer = Vec::new();
    
    parse_operator!(expressions, buffer, Mul);

    expressions = buffer.into_iter();
    buffer = Vec::new();

    parse_operator!(expressions, buffer, Div);

    expressions = buffer.into_iter();
    buffer = Vec::new();

    parse_operator!(expressions, buffer, Add);

    expressions = buffer.into_iter();
    buffer = Vec::new();

    parse_operator!(expressions, buffer, Sub);

    Err(ParsingError::ExpectedOperation)
}

pub fn parse(tokens: Vec<Token>) -> Result<ParseTree, ParsingError> {
    parse_expressions(tokens.into_iter().map(|token| Expression::Token(token)).collect())
}
