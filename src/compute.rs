use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

use crate::tokenizer::Token;
use crate::parser::ParseTree;

fn compute(tree: Option<Box<ParseTree>>) -> Option<Decimal> {
    match tree {
        Some(tree) => {
            match tree.token {
                Token::Add => Some(compute(tree.left)? + compute(tree.right)?),
                Token::Sub => Some(compute(tree.left)? - compute(tree.right)?),
                Token::Mul => Some(compute(tree.left)? * compute(tree.right)?),
                Token::Div => Some(compute(tree.left)? / compute(tree.right)?),
                Token::Exp => Decimal::from_f64(compute(tree.left)?.to_f64()?.powf(compute(tree.right)?.to_f64()?)),
                Token::Literal(x) => Some(x),
                _ => None,
            }
        },
        None => {
            Some(dec!(0))
        }
    }
}

pub fn compute_tree(tree: ParseTree) -> Option<Decimal> {
    compute(Some(Box::new(tree)))
}
