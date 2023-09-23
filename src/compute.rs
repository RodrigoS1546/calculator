use std::fmt::Display;

use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

use crate::parser::ParseTree;
use crate::tokenizer::Token;

pub enum ComputeError {
    Overflow,
    DivByZero,
    LogBaseZero,
    NotReal,
    NoAns,
    Unknown,
}

impl Display for ComputeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Overflow => write!(f, "Overflow"),
            Self::DivByZero => write!(f, "Division by zero"),
            Self::LogBaseZero => write!(f, "Log base zero"),
            Self::NotReal => write!(f, "Not real"),
            Self::NoAns => write!(f, "No previous answer"),
            Self::Unknown => write!(f, "Unkown"),
        }
    }
}

fn compute(tree: Option<Box<ParseTree>>, ans: Option<Decimal>) -> Result<Decimal, ComputeError> {
    match tree {
        Some(tree) => match tree.token {
            Token::Add => compute(tree.left, ans)?
                .checked_add(compute(tree.right, ans)?)
                .ok_or(ComputeError::Overflow),
            Token::Sub => compute(tree.left, ans)?
                .checked_sub(compute(tree.right, ans)?)
                .ok_or(ComputeError::Overflow),
            Token::Mul => compute(tree.left, ans)?
                .checked_mul(compute(tree.right, ans)?)
                .ok_or(ComputeError::Overflow),
            Token::Div => {
                let right = compute(tree.right, ans)?;
                if right.is_zero() {
                    return Err(ComputeError::DivByZero);
                }
                compute(tree.left, ans)?
                    .checked_div(right)
                    .ok_or(ComputeError::Overflow)
            }
            Token::Pow => compute(tree.left, ans)?
                .checked_powd(compute(tree.right, ans)?)
                .ok_or(ComputeError::Overflow),
            Token::Sin => compute(tree.left, ans)?
                .checked_sin()
                .ok_or(ComputeError::Overflow),
            Token::Cos => compute(tree.left, ans)?
                .checked_cos()
                .ok_or(ComputeError::Overflow),
            Token::Tan => compute(tree.left, ans)?
                .checked_tan()
                .ok_or(ComputeError::Overflow),
            Token::Ln => compute(tree.left, ans)?
                .checked_ln()
                .ok_or(ComputeError::Overflow),
            Token::Log => {
                if tree.right.is_none() {
                    let input = compute(tree.left, ans)?;
                    if input.is_zero() {
                        return Err(ComputeError::Overflow);
                    }
                    input.checked_log10().ok_or(ComputeError::NotReal)
                } else {
                    let base = compute(tree.left, ans)?;
                    if base.is_zero() {
                        return Err(ComputeError::LogBaseZero);
                    }
                    let val = compute(tree.right, ans)?;
                    if val.is_zero() {
                        return Err(ComputeError::Overflow);
                    }
                    val.checked_log10()
                        .ok_or(ComputeError::NotReal)?
                        .checked_div(base.checked_log10().ok_or(ComputeError::NotReal)?)
                        .ok_or(ComputeError::Overflow)
                }
            }
            Token::Sqrt => compute(tree.left, ans)?.sqrt().ok_or(ComputeError::NotReal),
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
