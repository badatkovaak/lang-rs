#![allow(dead_code)]
pub mod lexer;
pub mod parser;
pub mod interpreter;

use lexer::lex;


fn main() {
    let toks = lex("-. + 3 - 10^3/2");
    println!("Hello, world!");
    println!("{:?}", toks);
}
