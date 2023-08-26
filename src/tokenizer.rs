use rust_decimal::prelude::*;

#[derive(Debug, Default, Clone, Copy)]
pub enum Token {
    Add,
    Sub,
    Mul,
    Div,
    Exp,
    Sin,
    Cos,
    OpenParenthesis,
    CloseParenthesis,
    Literal(Decimal),
    Ans,
    PI,
    #[default]
    Invalid,
}

impl From<char> for Token {
    fn from(value: char) -> Self {
        match value {
            '+' => Self::Add,
            '-' => Self::Sub,
            'x' | '*' => Self::Mul,
            ':' | '/' => Self::Div,
            '^' => Self::Exp,
            '(' => Self::OpenParenthesis,
            ')' => Self::CloseParenthesis,
            _ => Self::Invalid,
        }
    }
}

impl From<String> for Token {
    fn from(value: String) -> Self {
        match value.parse::<Decimal>() {
            Ok(x) => Self::Literal(x),
            Err(_) => {
                if value.eq_ignore_ascii_case("ans") {
                    Self::Ans
                } else if value.eq_ignore_ascii_case("pi") || value.eq_ignore_ascii_case("π") {
                    Self::PI
                } else if value.eq_ignore_ascii_case("sin") {
                    Self::Sin
                } else if value.eq_ignore_ascii_case("cos") {
                    Self::Cos
                } else {
                    Self::Invalid
                }
            }
        }
    }
}

impl Token {
    fn is_value(&self) -> bool {
        match self {
            Token::Literal(_) | Token::PI | Token::Ans => true,
            _ => false,
        }
    }
}

pub fn tokenize(source: String) -> Option<Vec<Token>> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut iterator = source.chars().peekable();

    while let Some(c) = iterator.next() {
        if c.is_whitespace() {
            continue;
        } else if c.is_alphabetic() && c != 'x' {
            let mut literal = String::with_capacity(3);
            literal.push(c);
            while let Some(&d) = iterator.peek() {
                if d.is_alphabetic() && d != 'x' {
                    literal.push(iterator.next().unwrap_or_default());
                } else {
                    break;
                }
            }
            let token = literal.into();
            if let Token::Invalid = token {
                return None;
            }
            if let Some(prev) = tokens.last() {
                if prev.is_value() {
                    tokens.push(Token::Mul);
                }
            }
            tokens.push(token);
        } else if c.is_digit(10) {
            // tokenizing a decimal literal.
            let mut literal = String::new();
            literal.push(c);
            let mut dot_appeared = false;
            while let Some(&d) = iterator.peek() {
                if d == '.' {
                    if dot_appeared {
                        return None;
                    } else {
                        dot_appeared = true;
                    }
                    literal.push(iterator.next().unwrap_or_default());
                    continue;
                }
                if !d.is_digit(10) {
                    break;
                }
                literal.push(iterator.next().unwrap_or_default());
            }
            let token = literal.into();
            if let Token::Invalid = token {
                return None;
            }
            tokens.push(token);
        } else {
            let token = c.into();
            if let Token::Invalid = token {
                return None;
            }
            if let Token::OpenParenthesis = token {
                if let Some(prev) = tokens.last() {
                    if prev.is_value() {
                        tokens.push(Token::Mul);
                    }
                }
            }
            tokens.push(token);
        }
    }

    Some(tokens)
}
