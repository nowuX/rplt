use rplt::expr_parser::{exec_pattern, expr_matcher, tokens_to_expr};
use rplt::{Expr, Op, Token};
use test_case::test_case;
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

#[test_case(Token::OpenParen; "open_paren")]
#[test_case(Token::CloseParen; "close_paren")]
#[test_case(Token::Op(Op::Not); "not_op")]
#[test_case(Token::Var(String::from("r")); "var_r")]
fn expr_matcher_returns_none(token: Token) {
    assert_eq!(
        expr_matcher(
            &token,
            Box::new(Expr::Var(String::from("p"))),
            Box::new(Expr::Var(String::from("q")))
        ),
        None,
    );
}

#[test_case(Token::Op(Op::Or), Expr::Or; "or_op")]
#[test_case(Token::Op(Op::And), Expr::And; "and_op")]
#[test_case(Token::Op(Op::Conditional), Expr::Conditional; "conditional_op")]
#[test_case(Token::Op(Op::BiConditional), Expr::BiConditional; "biconditional_op")]
fn expr_matcher_creates_expr(token: Token, expected_variant: fn(Box<Expr>, Box<Expr>) -> Expr) {
    let p = Box::new(Expr::Var(String::from("p")));
    let q = Box::new(Expr::Var(String::from("q")));
    let expected = Some(expected_variant(
        Box::new(Expr::Var(String::from("p"))),
        Box::new(Expr::Var(String::from("q")))
    ));

    assert_eq!(expr_matcher(&token, p, q), expected);
}