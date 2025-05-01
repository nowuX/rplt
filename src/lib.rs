pub mod eval;
pub mod expr_parser;
pub mod table;
pub mod token_parser;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Var(String),
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
    Var(String),
    Not(Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Conditional(Box<Expr>, Box<Expr>),
    BiConditional(Box<Expr>, Box<Expr>),
}

#[expect(unused)]
pub struct Value {
    p: bool,
    new: bool,
    q: bool,
}

impl Value {
    pub fn new(p: bool, new: bool, q: bool) -> Self {
        Self { p, new, q }
    }
}

use colored::Colorize;

impl Expr {
    pub fn to_string(&self, first: bool) -> String {
        match self {
            Expr::Var(p) => p.to_string(),
            Expr::Not(p) => format!(
                "{0} {1}",
                match first {
                    true => "~".cyan().bold(),
                    false => "~".normal(),
                },
                p.to_string(false)
            ),
            Expr::Or(p, q) => format!(
                "({1} {0} {2})",
                match first {
                    true => "∨".cyan().bold(),
                    false => "∨".normal(),
                },
                p.to_string(false),
                q.to_string(false)
            ),
            Expr::And(p, q) => format!(
                "({1} {0} {2})",
                match first {
                    true => "∧".cyan().bold(),
                    false => "∧".normal(),
                },
                p.to_string(false),
                q.to_string(false)
            ),
            Expr::Conditional(p, q) => {
                format!(
                    "({1} {0} {2})",
                    match first {
                        true => "->".cyan().bold(),
                        false => "->".normal(),
                    },
                    p.to_string(false),
                    q.to_string(false)
                )
            }
            Expr::BiConditional(p, q) => {
                format!(
                    "({1} {0} {2})",
                    match first {
                        true => "<->".cyan().bold(),
                        false => "<->".normal(),
                    },
                    p.to_string(false),
                    q.to_string(false)
                )
            }
        }
    }
}
