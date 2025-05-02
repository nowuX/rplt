use crate::{Expr, Value};
use std::collections::HashMap;

pub fn eval(expr: &Expr, ctx: &mut HashMap<String, Vec<Value>>) -> Vec<Value> {
    let result = match expr {
        Expr::Var(p) => match ctx.get(&p.to_string()) {
            None => panic!("Undefined variable: {}", p),
            Some(p) => p.clone(),
        },
        Expr::Not(p) => {
            let p = match &**p {
                Expr::Var(p) => ctx.get(&p.clone()).unwrap(),
                _ => &eval(p, ctx),
            };
            not(p)
        }
        Expr::Or(p, q) => {
            let (p, q) = get_p_q(p, q, ctx);
            or(&p, &q)
        }
        Expr::And(p, q) => {
            let (p, q) = get_p_q(p, q, ctx);
            and(&p, &q)
        }
        Expr::Conditional(p, q) => {
            let (p, q) = get_p_q(p, q, ctx);
            conditional(&p, &q)
        }
        Expr::BiConditional(p, q) => {
            let (p, q) = get_p_q(p, q, ctx);
            bi_conditional(&p, &q)
        }
    };
    ctx.insert(expr.to_string(true), result.clone());
    result
}

pub fn not(p: &[Value]) -> Vec<Value> {
    p.iter()
        .map(|p| {
            let p = match p {
                Value::Value(p) => p,
                Value::Expr(_, p, _) => p,
            };
            Value::Expr(Some(*p), !p, None)
        })
        .collect()
}

pub fn or(p: &[Value], q: &[Value]) -> Vec<Value> {
    p.iter()
        .zip(q.iter())
        .map(|(p, q)| {
            let p = match p {
                Value::Value(p) => p,
                Value::Expr(_, p, _) => p,
            };
            let q = match q {
                Value::Value(q) => q,
                Value::Expr(_, q, _) => q,
            };
            Value::Expr(Some(*p), p | q, Some(*q))
        })
        .collect()
}

pub fn and(p: &[Value], q: &[Value]) -> Vec<Value> {
    p.iter()
        .zip(q.iter())
        .map(|(p, q)| {
            let p = match p {
                Value::Value(p) => p,
                Value::Expr(_, p, _) => p,
            };
            let q = match q {
                Value::Value(q) => q,
                Value::Expr(_, q, _) => q,
            };
            Value::Expr(Some(*p), p & q, Some(*q))
        })
        .collect()
}

pub fn conditional(p: &[Value], q: &[Value]) -> Vec<Value> {
    p.iter()
        .zip(q.iter())
        .map(|(p, q)| {
            let p = match p {
                Value::Value(p) => p,
                Value::Expr(_, p, _) => p,
            };
            let q = match q {
                Value::Value(q) => q,
                Value::Expr(_, q, _) => q,
            };
            Value::Expr(Some(*p), !matches!((p, q), (true, false)), Some(*q))
        })
        .collect()
}

pub fn bi_conditional(p: &[Value], q: &[Value]) -> Vec<Value> {
    p.iter()
        .zip(q.iter())
        .map(|(p, q)| {
            let p = match p {
                Value::Value(p) => p,
                Value::Expr(_, p, _) => p,
            };
            let q = match q {
                Value::Value(q) => q,
                Value::Expr(_, q, _) => q,
            };
            Value::Expr(
                Some(*p),
                matches!((p, q), (true, true) | (false, false)),
                Some(*q),
            )
        })
        .collect()
}

pub fn get_p_q(
    p: &Expr,
    q: &Expr,
    ctx: &mut HashMap<String, Vec<Value>>,
) -> (Vec<Value>, Vec<Value>) {
    let p = match p {
        Expr::Var(p) => ctx.get(&p.to_string()).unwrap().clone(),
        _ => eval(p, ctx),
    };
    let q = match q {
        Expr::Var(q) => ctx.get(&q.to_string()).unwrap().clone(),
        _ => eval(q, ctx),
    };
    (p, q)
}
