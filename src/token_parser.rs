use crate::Token;
use std::collections::{HashMap, HashSet};

pub fn parser(input: &str) -> Vec<Token> {
    // TODO change Var(char) to Var(String) to able to do ~ person instead or ~ p
    input
        .split_whitespace()
        .map(|word| match word {
            "~" => Token::Not,
            "or" => Token::Or,
            "and" => Token::And,
            "->" => Token::Conditional,
            "<->" => Token::BiConditional,
            "(" => Token::OpenParen,
            ")" => Token::CloseParen,
            &_ => Token::Var(word.chars().next().unwrap().to_string()),
        })
        .collect()
}

pub fn vars_values(tokens: &[Token]) -> HashMap<String, Vec<bool>> {
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
        let mut column = vec![];
        for row in &values {
            column.push(row[col]);
        }
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
