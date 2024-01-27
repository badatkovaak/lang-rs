#![allow(dead_code)]
// pub mod interpreter;
pub mod lexer;
pub mod lexer1;
pub mod parser;

use lexer1::lex;

fn main() {
    println!();
    let toks = lex("-. + 3 - 10^3/2");
    println!("{:?}", toks);
    println!();
}
