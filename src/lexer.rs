const KWORDS: &[&str] = &[
    "break", "const", "continue", "do", "else", "enum", "false", "float", "fn", "for", "if",
    "import", "int", "let", // "match",
    "return", "struct", "then", "true",
];

#[derive(Debug)]
pub enum Token {
    Illegal,
    Break,
    Const,
    Continue,
    Do,
    Else,
    Enum,
    False,
    Float,
    Func,
    For,
    If,
    Int,
    Let,
    // Match,
    Return,
    Struct,
    Then,
    True,
    Id(Box<str>),
    IntLiteral(u64),
    FloatLiteral(f64),
    StringLiteral(Box<str>),
    Assign,
    Equal,
    NotEqual,
    LessThen,
    GreaterThen,
    LessTEq,
    GreaterTEq,
    And,
    Or,
    Plus,
    Minus,
    Mult,
    Div,
    // Caret,
    LParen,
    RParen,
    LSquare,
    RSquare,
    Colon,
    Comma,
}

#[derive(Debug)]
pub enum LexerError {
    IllegalCharacter,
    LexErr,
}

use LexerError as LE;
use Token as T;

pub fn tokenize(input: String) -> Result<Vec<Token>, LexerError> {
    let mut toks = vec![];
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            '+' => {
                toks.push(T::Plus);
                chars.next();
            }
            '-' => {
                toks.push(T::Minus);
                chars.next();
            }
            '/' => {
                toks.push(T::Div);
                chars.next();
            }
            '*' => {
                toks.push(T::Mult);
                chars.next();
            }
            ',' => {
                toks.push(T::Comma);
                chars.next();
            }
            '(' => {
                toks.push(T::LParen);
                chars.next();
            }
            ')' => {
                toks.push(T::RParen);
                chars.next();
            }
            '[' => {
                toks.push(T::LSquare);
                chars.next();
            }
            ']' => {
                toks.push(T::RSquare);
                chars.next();
            }
            ':' => {
                toks.push(T::Colon);
                chars.next();
            }
            '<' => {
                chars.next();
                if let Some(&d) = chars.peek() {
                    match d {
                        '=' => {
                            toks.push(T::LessTEq);
                            chars.next();
                        }
                        _ => {
                            toks.push(T::LessThen);
                        }
                    }
                }
            }
            '>' => {
                chars.next();
                if let Some(&d) = chars.peek() {
                    match d {
                        '=' => {
                            toks.push(T::GreaterTEq);
                            chars.next();
                        }
                        _ => {
                            toks.push(T::GreaterThen);
                        }
                    }
                }
            }
            '!' => {
                chars.next();
                if let Some(&d) = chars.peek() {
                    if d == '=' {
                        toks.push(T::NotEqual);
                        chars.next();
                    } else {
                        return Err(LE::IllegalCharacter);
                    }
                }
            }
            '=' => {
                chars.next();
                if let Some(&d) = chars.peek() {
                    match d {
                        '=' => {
                            toks.push(T::Equal);
                            chars.next();
                        }
                        _ => {
                            toks.push(T::Assign);
                        }
                    }
                }
            }
            a if a.is_ascii_alphabetic() => {}
            _ => {
                return Err(LE::LexErr);
            }
        }
    }
    Ok(toks)
}
