use rplt::token_parser::{token_parser, vars_values};
use rplt::{Op, Token, Value};
use std::collections::HashMap;

#[test]
fn simple_str_parser() {
    assert_eq!(
        vec![
            Token::Var("p".to_string()),
            Token::Op(Op::Conditional),
            Token::Var("q".to_string())
        ],
        token_parser("p -> q")
    );
    assert_eq!(
        vec![
            Token::Var("p".to_string()),
            Token::Op(Op::And),
            Token::Var("q".to_string())
        ],
        token_parser("p and q")
    );
    assert_eq!(
        vec![
            Token::Op(Op::Not),
            Token::Var("p".to_string()),
            Token::Op(Op::Or),
            Token::Var("q".to_string())
        ],
        token_parser("~ p or q")
    );
    assert_eq!(
        vec![
            Token::OpenParen,
            Token::Var("p".to_string()),
            Token::Op(Op::And),
            Token::Var("q".to_string()),
            Token::CloseParen,
            Token::Op(Op::Conditional),
            Token::Var("r".to_string())
        ],
        token_parser("( p and q ) -> r")
    );
}

#[test]
fn long_str_parser() {
    assert_eq!(
        vec![
            Token::OpenParen,
            Token::Var("p".to_string()),
            Token::Op(Op::Conditional),
            Token::Var("q".to_string()),
            Token::CloseParen,
            Token::Op(Op::And),
            Token::OpenParen,
            Token::Var("q".to_string()),
            Token::Op(Op::Conditional),
            Token::Var("r".to_string()),
            Token::CloseParen
        ],
        token_parser("( p -> q ) and ( q -> r )")
    );
}

#[test]
fn vars_values_test() {
    assert_eq!(
        vars_values(&[Token::Var(String::from("p"))]),
        HashMap::from([(
            String::from("p"),
            vec![Value::Value(true), Value::Value(false)]
        )])
    );
    assert_eq!(
        vars_values(&[Token::Var(String::from("p")), Token::Var(String::from("q"))]),
        HashMap::from([
            (
                String::from("p"),
                vec![
                    Value::Value(true),
                    Value::Value(true),
                    Value::Value(false),
                    Value::Value(false)
                ]
            ),
            (
                String::from("q"),
                vec![
                    Value::Value(true),
                    Value::Value(false),
                    Value::Value(true),
                    Value::Value(false)
                ]
            )
        ])
    );
}
