use rplt::eval::{and, bi_conditional, conditional, eval, get_p_q, not, or};
use rplt::token_parser::vars_values;
use rplt::{Expr, Token};

#[test]
pub fn eval_test() {
    let mut ctx = vars_values(&[Token::Var(String::from("p")), Token::Var(String::from("q"))]);
    assert_eq!(
        vec![true, false, false, true],
        eval(&Expr::BiConditional(
            Box::new(Expr::Var(String::from("p"))),
            Box::new(Expr::Var(String::from("q")))
        ), &mut ctx)
    );
}

#[test]
fn not_test() {
    assert_eq!(vec![true], not(&[false]));
    assert_eq!(vec![false], not(&[true]));
}

#[test]
fn or_test() {
    assert_eq!(or(&[true], &[true]), vec![true]);
    assert_eq!(or(&[true], &[false]), vec![true]);
    assert_eq!(or(&[false], &[true]), vec![true]);
    assert_eq!(or(&[false], &[false]), vec![false]);
}

#[test]
fn and_test() {
    assert_eq!(and(&[true], &[true]), vec![true]);
    assert_eq!(and(&[true], &[false]), vec![false]);
    assert_eq!(and(&[false], &[true]), vec![false]);
    assert_eq!(and(&[false], &[false]), vec![false]);
}

#[test]
fn conditional_test() {
    assert_eq!(conditional(&[true], &[true]), vec![true]);
    assert_eq!(conditional(&[true], &[false]), vec![false]);
    assert_eq!(conditional(&[false], &[true]), vec![true]);
    assert_eq!(conditional(&[false], &[false]), vec![true]);
}

#[test]
fn bi_conditional_test() {
    assert_eq!(bi_conditional(&[true], &[true]), vec![true]);
    assert_eq!(bi_conditional(&[true], &[false]), vec![false]);
    assert_eq!(bi_conditional(&[false], &[true]), vec![false]);
    assert_eq!(bi_conditional(&[false], &[false]), vec![true]);
}

#[test]
fn get_p_q_test() {
    let mut ctx = vars_values(&[Token::Var(String::from("p")), Token::Var(String::from("q"))]);
    let p = Box::new(Expr::Var("p".to_string()));
    let q = Box::new(Expr::Var("q".to_string()));
    assert_eq!(
        get_p_q(&p, &q, &mut ctx),
        (
            vec![true, true, false, false],
            vec![true, false, true, false]
        )
    );
}
