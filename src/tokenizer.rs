use rust_decimal::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum Token {
    Add,
    Sub,
    ImplMul,
    Mul,
    Div,
    Pow,
    Sin,
    Cos,
    Tan,
    Exp,
    Ln,
    Log,
    Sqrt,
    Factorial,
    OpenParenthesis,
    CloseParenthesis,
    Comma,
    Literal(Decimal),
    Ans,
    PI,
    E,
}

impl TryFrom<char> for Token {
    type Error = ();

    fn try_from(value: char) -> Result<Self, ()> {
        match value {
            '+' => Ok(Self::Add),
            '-' => Ok(Self::Sub),
            '*' => Ok(Self::Mul),
            ':' | '/' => Ok(Self::Div),
            '^' => Ok(Self::Pow),
            '√' => Ok(Self::Sqrt),
            '!' => Ok(Self::Factorial),
            '(' => Ok(Self::OpenParenthesis),
            ')' => Ok(Self::CloseParenthesis),
            ',' => Ok(Self::Comma),
            _ => Err(()),
        }
    }
}

impl TryFrom<String> for Token {
    type Error = ();

    fn try_from(value: String) -> Result<Self, ()> {
        match value.parse::<Decimal>() {
            Ok(x) => Ok(Self::Literal(x)),
            Err(_) => {
                if value.eq_ignore_ascii_case("ans") {
                    Ok(Self::Ans)
                } else if value.eq_ignore_ascii_case("pi") || value.eq_ignore_ascii_case("π") {
                    Ok(Self::PI)
                } else if value.eq_ignore_ascii_case("e") {
                    Ok(Self::E)
                } else if value.eq_ignore_ascii_case("sin") {
                    Ok(Self::Sin)
                } else if value.eq_ignore_ascii_case("cos") {
                    Ok(Self::Cos)
                } else if value.eq_ignore_ascii_case("tan") {
                    Ok(Self::Tan)
                } else if value.eq_ignore_ascii_case("ln") {
                    Ok(Self::Ln)
                } else if value.eq_ignore_ascii_case("log") {
                    Ok(Self::Log)
                } else if value.eq_ignore_ascii_case("exp") {
                    Ok(Self::Exp)
                } else if value.eq_ignore_ascii_case("sqrt") {
                    Ok(Self::Sqrt)
                } else {
                    Err(())
                }
            }
        }
    }
}

impl Token {
    pub fn is_value(&self) -> bool {
        match self {
            Token::Literal(_) | Token::PI | Token::E | Token::Ans => true,
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
        } else if c.is_alphabetic() {
            let mut literal = String::with_capacity(3);
            literal.push(c);
            while let Some(&d) = iterator.peek() {
                if d.is_alphabetic() {
                    literal.push(iterator.next().unwrap_or_default());
                } else {
                    break;
                }
            }
            let token = literal.try_into().ok()?;
            if let Some(prev) = tokens.last() {
                if prev.is_value() {
                    tokens.push(Token::ImplMul);
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
            let token = literal.try_into().ok()?;
            tokens.push(token);
        } else {
            let token = c.try_into().ok()?;
            if let Token::OpenParenthesis | Token::Sqrt = token {
                if let Some(prev) = tokens.last() {
                    if prev.is_value() {
                        tokens.push(Token::ImplMul);
                    }
                }
            }
            tokens.push(token);
        }
    }

    Some(tokens)
}
