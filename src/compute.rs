use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

use crate::parser::ParseTree;
use crate::tokenizer::Token;

fn compute(tree: Option<Box<ParseTree>>, ans: Option<Decimal>) -> Option<Decimal> {
    match tree {
        Some(tree) => match tree.token {
            Token::Add => Some(compute(tree.left, ans)? + compute(tree.right, ans)?),
            Token::Sub => Some(compute(tree.left, ans)? - compute(tree.right, ans)?),
            Token::Mul => Some(compute(tree.left, ans)? * compute(tree.right, ans)?),
            Token::Div => Some(compute(tree.left, ans)? / compute(tree.right, ans)?),
            Token::Exp => Some(compute(tree.left, ans)?.powd(compute(tree.right, ans)?)),
            Token::Literal(x) => Some(x),
            Token::PI => Some(Decimal::PI),
            Token::Ans => ans,
            _ => None,
        },
        None => Some(dec!(0)),
    }
}

pub fn compute_tree(tree: ParseTree, ans: Option<Decimal>) -> Option<Decimal> {
    compute(Some(Box::new(tree)), ans)
}
