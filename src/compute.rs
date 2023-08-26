use std::fmt::Display;

use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

use crate::parser::ParseTree;
use crate::tokenizer::Token;

pub enum ComputeError {
    Overflow,
    DivByZero,
    NoAns,
    Unknown,
}

impl Display for ComputeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Overflow => write!(f, "Overflow"),
            Self::DivByZero => write!(f, "Division by zero"),
            Self::NoAns => write!(f, "No previous answer"),
            Self::Unknown => write!(f, "Unkown"),
        }
    }
}

macro_rules! compute_function {
    ($value:expr,$op:ident) => {
        match $value.$op() {
            Some(x) => Ok(x),
            None => Err(ComputeError::Overflow),
        }
    };
}

macro_rules! compute_operation {
    ($left:expr,$right:expr,$op:ident) => {
        match $left.$op($right) {
            Some(x) => Ok(x),
            None => Err(ComputeError::Overflow),
        }
    };
}

fn compute(tree: Option<Box<ParseTree>>, ans: Option<Decimal>) -> Result<Decimal, ComputeError> {
    match tree {
        Some(tree) => match tree.token {
            Token::Add => compute_operation!(
                compute(tree.left, ans)?,
                compute(tree.right, ans)?,
                checked_add
            ),
            Token::Sub => compute_operation!(
                compute(tree.left, ans)?,
                compute(tree.right, ans)?,
                checked_sub
            ),
            Token::Mul => compute_operation!(
                compute(tree.left, ans)?,
                compute(tree.right, ans)?,
                checked_mul
            ),
            Token::Div => {
                let right = compute(tree.right, ans)?;
                if right.is_zero() {
                    return Err(ComputeError::DivByZero);
                }
                compute_operation!(compute(tree.left, ans)?, right, checked_div)
            }
            Token::Pow => compute_operation!(
                compute(tree.left, ans)?,
                compute(tree.right, ans)?,
                checked_powd
            ),
            Token::Sin => compute_function!(compute(tree.left, ans)?, checked_sin),
            Token::Cos => compute_function!(compute(tree.left, ans)?, checked_cos),
            Token::Tan => compute_function!(compute(tree.left, ans)?, checked_tan),
            Token::Ln => compute_function!(compute(tree.left, ans)?, checked_ln),
            Token::Log => compute_function!(compute(tree.left, ans)?, checked_log10),
            Token::Literal(x) => Ok(x),
            Token::PI => Ok(Decimal::PI),
            Token::E => Ok(Decimal::E),
            Token::Ans => ans.ok_or(ComputeError::NoAns),
            _ => Err(ComputeError::Unknown),
        },
        None => Ok(dec!(0)),
    }
}

pub fn compute_tree(tree: ParseTree, ans: Option<Decimal>) -> Result<Decimal, ComputeError> {
    compute(Some(Box::new(tree)), ans)
}
