use rplt::expr_parser::{exec_pattern, expr_matcher, tokens_to_expr};
use rplt::{Expr, Op, Token};
use std::sync::Arc;

#[test]
fn token_to_expr_test() {
    assert_eq!(
        tokens_to_expr(&mut vec![Token::Var(String::from("p"))],),
        Expr::Var(String::from("p")),
    );
}

#[test]
fn exec_pattern_test() {
    let mut tokens = vec![Token::Op(Op::Not), Token::Var(String::from("p"))];
    exec_pattern(
        &[(
            2,
            Arc::new(|slice: &[Token]| match slice {
                [Token::Op(Op::Not), Token::Var(p)] => Some(Expr::Not(Box::new(Expr::Var(p.clone())))),
                _ => None,
            }),
        )],
        &mut tokens,
    );
    assert_eq!(
        tokens,
        vec![Token::Expr(Expr::Not(Box::new(Expr::Var(String::from(
            "p"
        )))))]
    );
}

#[test]
fn expr_matcher_test() {
    for i in [
        Token::OpenParen,
        Token::CloseParen,
        Token::Op(Op::Not),
        Token::Var(String::from("r")),
    ] {
        assert_eq!(
            expr_matcher(
                &i,
                Box::new(Expr::Var(String::from("p"))),
                Box::new(Expr::Var(String::from("q")))
            ),
            None,
        );
    }

    assert_eq!(
        expr_matcher(
            &Token::Op(Op::Or),
            Box::new(Expr::Var(String::from("p"))),
            Box::new(Expr::Var(String::from("q")))
        ),
        Some(Expr::Or(
            Box::new(Expr::Var(String::from("p"))),
            Box::new(Expr::Var(String::from("q")))
        )),
    );
    assert_eq!(
        expr_matcher(
            &Token::Op(Op::And),
            Box::new(Expr::Var(String::from("p"))),
            Box::new(Expr::Var(String::from("q")))
        ),
        Some(Expr::And(
            Box::new(Expr::Var(String::from("p"))),
            Box::new(Expr::Var(String::from("q")))
        )),
    );
    assert_eq!(
        expr_matcher(
            &Token::Op(Op::Conditional),
            Box::new(Expr::Var(String::from("p"))),
            Box::new(Expr::Var(String::from("q")))
        ),
        Some(Expr::Conditional(
            Box::new(Expr::Var(String::from("p"))),
            Box::new(Expr::Var(String::from("q")))
        )),
    );
    assert_eq!(
        expr_matcher(
            &Token::Op(Op::BiConditional),
            Box::new(Expr::Var(String::from("p"))),
            Box::new(Expr::Var(String::from("q")))
        ),
        Some(Expr::BiConditional(
            Box::new(Expr::Var(String::from("p"))),
            Box::new(Expr::Var(String::from("q")))
        )),
    );
}
