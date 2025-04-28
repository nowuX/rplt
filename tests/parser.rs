use rplt::token_parser::parser;
use rplt::Token;

/**
p -> q
p and q
~p or q
(p and q) -> r
**/
#[test]
fn simple_str_parser() {
    assert_eq!(
        vec![Token::Var("p".to_string()), Token::Conditional, Token::Var("q".to_string())],
        parser("p -> q")
    );
    assert_eq!(
        vec![Token::Var("p".to_string()), Token::And, Token::Var("q".to_string())],
        parser("p and q")
    );
    assert_eq!(
        vec![Token::Not, Token::Var("p".to_string()), Token::Or, Token::Var("q".to_string())],
        parser("~ p or q")
    );
    assert_eq!(
        vec![
            Token::OpenParen,
            Token::Var("p".to_string()),
            Token::And,
            Token::Var("q".to_string()),
            Token::CloseParen,
            Token::Conditional,
            Token::Var("r".to_string())
        ],
        parser("( p and q ) -> r")
    );
}

#[test]
fn long_str_parser() {
    assert_eq!(
        vec![
            Token::OpenParen,
            Token::Var("p".to_string()),
            Token::Conditional,
            Token::Var("q".to_string()),
            Token::CloseParen,
            Token::And,
            Token::OpenParen,
            Token::Var("q".to_string()),
            Token::Conditional,
            Token::Var("r".to_string()),
            Token::CloseParen
        ],
        parser("( p -> q ) and ( q -> r )")
    );
    // ~(p and q) or (r and s)
    //     ((p or q) and (~r or s)) -> t
    // p <-> (q or ~r)
}
