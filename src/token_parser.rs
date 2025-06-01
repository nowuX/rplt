use crate::{Op, Token, Value};
use std::collections::{HashMap, HashSet};

pub fn token_parser(input: &str) -> Vec<Token> {
    // TODO increment parser
    input
        .split_whitespace()
        .map(|word| match word {
            "~" => Token::Op(Op::Not),
            "or" => Token::Op(Op::Or),
            "and" => Token::Op(Op::And),
            "->" => Token::Op(Op::Conditional),
            "<->" => Token::Op(Op::BiConditional),
            "(" => Token::OpenParen,
            ")" => Token::CloseParen,
            &_ => Token::Var(word.chars().next().unwrap().to_string()),
        })
        .collect()
}

pub fn vars_values(tokens: &[Token]) -> HashMap<String, Vec<Value>> {
    let vars = get_vars(tokens);

    let n: usize = vars.len();
    let total = 1 << n;
    let values = {
        let mut x = (0..total)
            .map(|i| {
                (0..n)
                    .map(|j| (i >> (n - 1 - j) & 1 == 1))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<Vec<_>>>();
        x.reverse();
        x
    };

    let mut map = HashMap::new();
    for col in 0..values.first().unwrap().len() {
        let column = values
            .iter()
            .map(|row| Value::Value(row[col]))
            .collect::<Vec<_>>();
        map.insert(vars[col].to_string(), column);
    }
    map
}

pub fn get_vars(tokens: &[Token]) -> Vec<String> {
    let mut vars = HashSet::new();
    for t in tokens {
        if let Token::Var(c) = t {
            vars.insert(c.clone());
        }
    }
    let mut vars = vars.into_iter().collect::<Vec<_>>();
    vars.sort_unstable();
    vars
}
