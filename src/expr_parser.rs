use crate::{Expr, Op, Token};
use once_cell::sync::Lazy;
use std::sync::Arc;

type Pattern = (usize, Arc<dyn Fn(&[Token]) -> Option<Expr> + Send + Sync>);

pub fn tokens_to_expr(tokens: &mut Vec<Token>) -> Expr {
    if tokens.len() == 1 {
        if let Some(Token::Var(p)) = tokens.first() {
            return Expr::Var(p.to_owned());
        }
        panic!("Unexpected token in single token expression");
    }

    assert!(!tokens.is_empty(), "Tokens cannot be empty");

    let parsers = [
        &*NOT_PARSER,
        &*VAR_TO_VAR,
        &*VAR_TO_EXPR,
        &*EXPR_TO_VAR,
        &*EXPR_TO_EXPR,
    ];

    while !tokens.iter().all(|t| matches!(t, Token::Expr(_))) {
        let tokens_before = tokens.clone();

        parsers
            .iter()
            .for_each(|parser| exec_pattern(parser, tokens));

        if tokens == &tokens_before {
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
static NOT_PARSER: Lazy<Vec<Pattern>> = Lazy::new(|| {
    vec![
        (
            2,
            Arc::new(|slice: &[Token]| match slice {
                [Token::Op(Op::Not), Token::Var(p)] => {
                    Some(Expr::Not(Box::new(Expr::Var(p.clone()))))
                }
                _ => None,
            }),
        ),
        (
            4,
            Arc::new(|slice: &[Token]| match slice {
                [
                    Token::Op(Op::Not),
                    Token::OpenParen,
                    Token::Expr(p),
                    Token::CloseParen,
                ] => Some(Expr::Not(Box::new(p.clone()))),
                _ => None,
            }),
        ),
    ]
});

/// - p to p
static VAR_TO_VAR: Lazy<Vec<Pattern>> = Lazy::new(|| {
    vec![(
        3,
        Arc::new(|slice: &[Token]| match slice {
            [Token::Var(p), t, Token::Var(q)] => {
                let p = Box::new(Expr::Var(p.clone()));
                let q = Box::new(Expr::Var(q.clone()));
                expr_matcher(t, p, q)
            }
            _ => None,
        }),
    )]
});

/// - p to ( expr )
/// - p to expr
static VAR_TO_EXPR: Lazy<Vec<Pattern>> = Lazy::new(|| {
    vec![
        (
            5,
            Arc::new(|slice: &[Token]| match slice {
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
            Arc::new(|slice: &[Token]| match slice {
                [Token::Var(p), t, Token::Expr(q)] => {
                    let p = Box::new(Expr::Var(p.to_owned()));
                    let q = Box::new(q.clone());
                    expr_matcher(t, p, q)
                }
                _ => None,
            }),
        ),
    ]
});

/// - ( expr ) to var
/// - expr to var
static EXPR_TO_VAR: Lazy<Vec<Pattern>> = Lazy::new(|| {
    vec![
        (
            5,
            Arc::new(|slice: &[Token]| match slice {
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
            Arc::new(|slice: &[Token]| match slice {
                [Token::Expr(p), t, Token::Var(q)] => {
                    let p = Box::new(p.clone());
                    let q = Box::new(Expr::Var(q.to_owned()));
                    expr_matcher(t, p, q)
                }
                _ => None,
            }),
        ),
    ]
});

/// - expr to expr
/// - ( expr ) to expr
/// - expr to ( expr )
/// - ( expr ) to ( expr )
static EXPR_TO_EXPR: Lazy<Vec<Pattern>> = Lazy::new(|| {
    vec![
        (
            5,
            Arc::new(|slice: &[Token]| match slice {
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
            Arc::new(|slice: &[Token]| match slice {
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
            Arc::new(|slice: &[Token]| match slice {
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
            Arc::new(|slice: &[Token]| match slice {
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
});

pub fn exec_pattern(patterns: &[Pattern], tokens: &mut Vec<Token>) {
    for (token_size, pattern_fn) in patterns {
        let mut i = 0;
        while i + token_size - 1 < tokens.len() {
            let expr = pattern_fn(&tokens[i..i + token_size]);
            replace_expr(expr, &mut i, tokens, *token_size);
        }
    }
}

pub fn expr_matcher(t: &Token, p: Box<Expr>, q: Box<Expr>) -> Option<Expr> {
    match t {
        Token::Op(op) => match op {
            Op::Or => Some(Expr::Or(p, q)),
            Op::And => Some(Expr::And(p, q)),
            Op::Conditional => Some(Expr::Conditional(p, q)),
            Op::BiConditional => Some(Expr::BiConditional(p, q)),
            _ => None,
        },
        _ => None,
    }
}

pub fn replace_expr(expr: Option<Expr>, i: &mut usize, tokens: &mut Vec<Token>, token_size: usize) {
    match expr {
        None => *i += 1,
        Some(expr) => {
            tokens.splice(*i..(*i + token_size), [Token::Expr(expr)]);
            *i = i.saturating_sub(1);
        }
    }
}
