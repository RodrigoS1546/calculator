use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

use crate::tokenizer::Token;
use crate::parser::ParseTree;

fn compute(tree: Option<Box<ParseTree>>, ans: Option<Decimal>) -> Option<Decimal> {
    match tree {
        Some(tree) => {
            match tree.token {
                Token::Add => Some(compute(tree.left, ans)? + compute(tree.right, ans)?),
                Token::Sub => Some(compute(tree.left, ans)? - compute(tree.right, ans)?),
                Token::Mul => Some(compute(tree.left, ans)? * compute(tree.right, ans)?),
                Token::Div => Some(compute(tree.left, ans)? / compute(tree.right, ans)?),
                Token::Exp => Decimal::from_f64(compute(tree.left, ans)?.to_f64()?.powf(compute(tree.right, ans)?.to_f64()?)),
                Token::Literal(x) => Some(x),
                Token::Ans => ans,
                _ => None,
            }
        },
        None => {
            Some(dec!(0))
        }
    }
}

pub fn compute_tree(tree: ParseTree, ans: Option<Decimal>) -> Option<Decimal> {
    compute(Some(Box::new(tree)), ans)
}
