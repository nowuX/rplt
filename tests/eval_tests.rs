use rplt::eval::{and, bi_conditional, conditional, eval, get_p_q, not, or};
use rplt::token_parser::vars_values;
use rplt::{Expr, Token, Value};

use test_case::test_case;

#[test]
pub fn eval_test() {
    let mut ctx = vars_values(&[Token::Var(String::from("p")), Token::Var(String::from("q"))]);
    let result = eval(
        &Expr::BiConditional(
            Box::new(Expr::Var(String::from("p"))),
            Box::new(Expr::Var(String::from("q"))),
        ),
        &mut ctx,
    )
    .iter()
    .map(|v| match v {
        Value::Value(p) => *p,
        Value::Expr(_, p, _) => *p,
    })
    .collect::<Vec<_>>();
    assert_eq!(vec![true, false, false, true], result)
}

#[test_case(false, true ; "not false")]
#[test_case(true, false ; "not true")]
fn not_test(x: bool, y: bool) {
    let x = not(&[Value::Value(x)]);
    let result = x.iter().map(|v| v.value()).collect::<Vec<_>>();
    assert_eq!(result, vec![y]);
}

#[test_case(true, true, true ; "true or true")]
#[test_case(true, false, true ; "true or false")]
#[test_case(false, true, true ; "false or true")]
#[test_case(false, false, false ; "false or false")]
fn or_test(p: bool, q: bool, r: bool) {
    let x = or(&[Value::Value(p)], &[Value::Value(q)]);
    let result = x.iter().map(|v| v.value()).collect::<Vec<_>>();
    assert_eq!(result, vec![r]);
}

#[test_case(true, true, true ; "true and true")]
#[test_case(true, false, false ; "true and false")]
#[test_case(false, true, false ; "false and true")]
#[test_case(false, false, false ; "false and false")]
fn and_test(p: bool, q: bool, r: bool) {
    let x = and(&[Value::Value(p)], &[Value::Value(q)]);
    let result = x.iter().map(|v| v.value()).collect::<Vec<_>>();
    assert_eq!(result, vec![r]);
}

#[test_case(true, true, true ; "true conditional true")]
#[test_case(true, false, false ; "true conditional false")]
#[test_case(false, true, true ; "false conditional true")]
#[test_case(false, false, true ; "false conditional false")]
fn conditional_test(p: bool, q: bool, r: bool) {
    let x = conditional(&[Value::Value(p)], &[Value::Value(q)]);
    let result = x.iter().map(|v| v.value()).collect::<Vec<_>>();
    assert_eq!(result, vec![r]);
}

#[test_case(true, true, true ; "true bi_conditional true")]
#[test_case(true, false, false ; "true bi_conditional false")]
#[test_case(false, true, false ; "false bi_conditional true")]
#[test_case(false, false, true ; "false bi_conditional false")]
fn bi_conditional_test(p: bool, q: bool, r: bool) {
    let x = bi_conditional(&[Value::Value(p)], &[Value::Value(q)]);
    let result = x.iter().map(|v| v.value()).collect::<Vec<_>>();
    assert_eq!(result, vec![r]);
}

#[test]
fn get_p_q_test() {
    let mut ctx = vars_values(&[Token::Var(String::from("p")), Token::Var(String::from("q"))]);
    let p = Box::new(Expr::Var("p".to_string()));
    let q = Box::new(Expr::Var("q".to_string()));
    let (p, q) = get_p_q(&p, &q, &mut ctx);
    let result = p
        .iter()
        .zip(q.iter())
        .map(|(p, q)| (p.value(), q.value()))
        .collect::<Vec<_>>();
    assert_eq!(
        result,
        vec![(true, true), (true, false), (false, true), (false, false)]
    );
}
