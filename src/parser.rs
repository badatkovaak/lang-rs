// use crate::lexer::Token;
pub enum Exp {
    Plus(Box<Exp>, Box<Term>),
    Minus(Box<Exp>,Box<Term>),
    Pure(Box<Term>)
}

pub enum Term{
    Mult(Box<Term>, Box<Factor>),
    Div(Box<Term>, Box<Factor>),
    Pure(Factor)
}

pub enum Factor {
    Power(Box<Factor>, Box<Exponent>),
    Pure(Box<Exponent>)
}

pub enum Exponent {
    Parens(Box<Exponent>),
    Value(f64),
}

// pub fn parse(input: Vec<Token>) -> Option<Exp> {
//     for i in input.iter() {
//         match i {
//             Token::Number()
//             _ => {}
//         }
//     }
//     None
// }