use crate::eval::eval;
use crate::expr_parser::tokens_to_expr;
use crate::table::generate_table;
use crate::token_parser::{vars_values, parser};

pub mod expr_parser;
pub mod table;
pub mod token_parser;
pub mod eval;
mod display;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Var(char),
    Not,
    Or,
    And,
    Conditional,
    BiConditional,
    OpenParen,
    CloseParen,
    Expr(Expr),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Var(char),
    Not(Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Conditional(Box<Expr>, Box<Expr>),
    BiConditional(Box<Expr>, Box<Expr>),
}

fn main() {
    let s = vec![
        "p -> ( p or q )",
        "( p and q ) and ~ p",
        "( p or q ) -> q"
    ];
    for i in s {
        let mut tokens = parser(i);
        let mut ctx = vars_values(&tokens);
        let expr = tokens_to_expr(&mut tokens);
        let _ = eval(&expr, &mut ctx);
        println!("{}\n", generate_table(&ctx));
    }
}
