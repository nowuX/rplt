use crate::{Expr, Token};

type Pattern = (usize, Box<dyn Fn(&[Token]) -> Option<Expr>>);

pub fn tokens_to_expr(tokens: &mut Vec<Token>) -> Expr {
    if tokens.len() == 1 {
        if let Some(Token::Var(p)) = tokens.first() {
            return Expr::Var(p.to_owned());
        }
        panic!("Unexpected token in single token expression");
    }

    assert!(!tokens.is_empty(), "Tokens cannot be empty");

    let parsers = [
        not_parser(),
        var_to_var(),
        var_to_expr(),
        expr_to_var(),
        expr_to_expr(),
    ];

    while !tokens.iter().all(|t| matches!(t, Token::Expr(_))) {
        let tokens_before = tokens.clone();

        parsers
            .iter()
            .for_each(|parser| exec_pattern(parser, tokens));

        if tokens == &tokens_before {
            println!("{tokens:#?}");
            panic!("Parser reached a state where no further transformations are possible");
        }
    }

    match tokens.first() {
        None => panic!("Failed to parse: empty token list"),
        Some(token) => match token {
            Token::Expr(e) => e.clone(),
            _ => panic!("Expected expression token at the end of parsing"),
        },
    }
}

/// - ~ p
/// - ~ ( expr )
fn not_parser() -> Vec<Pattern> {
    vec![
        (
            2,
            Box::new(|slice: &[Token]| match slice {
                [Token::Not, Token::Var(p)] => Some(Expr::Not(Box::new(Expr::Var(p.clone())))),
                _ => None,
            }),
        ),
        (
            4,
            Box::new(|slice: &[Token]| match slice {
                [
                    Token::Not,
                    Token::OpenParen,
                    Token::Expr(p),
                    Token::CloseParen,
                ] => Some(Expr::Not(Box::new(p.clone()))),
                _ => None,
            }),
        ),
    ]
}

/// - p to p
fn var_to_var() -> Vec<Pattern> {
    vec![(
        3,
        Box::new(|slice: &[Token]| match slice {
            [Token::Var(p), t, Token::Var(q)] => {
                let p = Box::new(Expr::Var(p.clone()));
                let q = Box::new(Expr::Var(q.clone()));
                expr_matcher(t, p, q)
            }
            _ => None,
        }),
    )]
}

/// - p to ( expr )
/// - p to expr
fn var_to_expr() -> Vec<Pattern> {
    vec![
        (
            5,
            Box::new(|slice: &[Token]| match slice {
                [
                    Token::Var(p),
                    t,
                    Token::OpenParen,
                    Token::Expr(q),
                    Token::CloseParen,
                ] => {
                    let p = Box::new(Expr::Var(p.clone()));
                    let q = Box::new(q.clone());
                    expr_matcher(t, p, q)
                }
                _ => None,
            }),
        ),
        (
            3,
            Box::new(|slice: &[Token]| match slice {
                [Token::Var(p), t, Token::Expr(q)] => {
                    let p = Box::new(Expr::Var(p.to_owned()));
                    let q = Box::new(q.clone());
                    expr_matcher(t, p, q)
                }
                _ => None,
            }),
        ),
    ]
}

/// - ( expr ) to var
/// - expr to var
fn expr_to_var() -> Vec<Pattern> {
    vec![
        (
            5,
            Box::new(|slice: &[Token]| match slice {
                [
                    Token::OpenParen,
                    Token::Expr(p),
                    Token::CloseParen,
                    t,
                    Token::Var(q),
                ] => {
                    let p = Box::new(p.clone());
                    let q = Box::new(Expr::Var(q.to_owned()));
                    expr_matcher(t, p, q)
                }
                _ => None,
            }),
        ),
        (
            3,
            Box::new(|slice: &[Token]| match slice {
                [Token::Expr(p), t, Token::Var(q)] => {
                    let p = Box::new(p.clone());
                    let q = Box::new(Expr::Var(q.to_owned()));
                    expr_matcher(t, p, q)
                }
                _ => None,
            }),
        ),
    ]
}

/// - expr to expr
/// - ( expr ) to expr
/// - expr to ( expr )
/// - ( expr ) to ( expr )
fn expr_to_expr() -> Vec<Pattern> {
    vec![
        (
            5,
            Box::new(|slice: &[Token]| match slice {
                [
                    Token::Expr(p),
                    t,
                    Token::OpenParen,
                    Token::Expr(q),
                    Token::CloseParen,
                ] => {
                    let p = Box::new(p.clone());
                    let q = Box::new(q.clone());
                    expr_matcher(t, p, q)
                }
                _ => None,
            }),
        ),
        (
            3,
            Box::new(|slice: &[Token]| match slice {
                [Token::Expr(p), t, Token::Expr(q)] => {
                    let p = Box::new(p.clone());
                    let q = Box::new(q.clone());
                    expr_matcher(t, p, q)
                }
                _ => None,
            }),
        ),
        (
            5,
            Box::new(|slice: &[Token]| match slice {
                [
                    Token::OpenParen,
                    Token::Expr(p),
                    Token::CloseParen,
                    t,
                    Token::Expr(q),
                ] => {
                    let p = Box::new(p.clone());
                    let q = Box::new(q.clone());
                    expr_matcher(t, p, q)
                }
                _ => None,
            }),
        ),
        (
            7,
            Box::new(|slice: &[Token]| match slice {
                [
                    Token::OpenParen,
                    Token::Expr(p),
                    Token::CloseParen,
                    t,
                    Token::OpenParen,
                    Token::Expr(q),
                    Token::CloseParen,
                ] => {
                    let p = Box::new(p.clone());
                    let q = Box::new(q.clone());
                    expr_matcher(t, p, q)
                }
                _ => None,
            }),
        ),
    ]
}

fn exec_pattern(patterns: &[Pattern], tokens: &mut Vec<Token>) {
    for (token_size, pattern_fn) in patterns {
        let mut i = 0;
        while i + token_size - 1 < tokens.len() {
            let expr = pattern_fn(&tokens[i..i + token_size]);
            replace_expr(expr, &mut i, tokens, *token_size);
        }
    }
}

fn expr_matcher(t: &Token, p: Box<Expr>, q: Box<Expr>) -> Option<Expr> {
    match t {
        Token::Or => Some(Expr::Or(p, q)),
        Token::And => Some(Expr::And(p, q)),
        Token::Conditional => Some(Expr::Conditional(p, q)),
        Token::BiConditional => Some(Expr::BiConditional(p, q)),
        _ => None,
    }
}

fn replace_expr(expr: Option<Expr>, i: &mut usize, tokens: &mut Vec<Token>, token_size: usize) {
    match expr {
        None => *i += 1,
        Some(expr) => {
            tokens.splice(*i..(*i + token_size), [Token::Expr(expr)]);
            *i = i.saturating_sub(1);
        }
    }
}
