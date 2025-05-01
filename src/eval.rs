use crate::Expr;
use std::collections::HashMap;

pub fn eval(expr: &Expr, ctx: &mut HashMap<String, Vec<bool>>) -> Vec<bool> {
    let result = match expr {
        Expr::Var(p) => ctx.get(&p.to_string()).unwrap().clone(),
        Expr::Not(p) => match &**p {
            Expr::Var(p) => not(ctx.get(&p.clone()).unwrap()),
            _ => not(&eval(p, ctx)),
        },
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

fn not(p: &[bool]) -> Vec<bool> {
    p.iter().map(|v| !v).collect()
}

fn or(p: &[bool], q: &[bool]) -> Vec<bool> {
    p.iter().zip(q.iter()).map(|(p, q)| p | q).collect()
}

fn and(p: &[bool], q: &[bool]) -> Vec<bool> {
    p.iter().zip(q.iter()).map(|(p, q)| p & q).collect()
}

fn conditional(p: &[bool], q: &[bool]) -> Vec<bool> {
    p.iter()
        .zip(q.iter())
        .map(|(p, q)| !matches!((p, q), (true, false)))
        .collect()
}

fn bi_conditional(p: &[bool], q: &[bool]) -> Vec<bool> {
    p.iter()
        .zip(q.iter())
        .map(|(p, q)| matches!((p, q), (true, true) | (false, false)))
        .collect()
}

fn get_p_q(p: &Expr, q: &Expr, ctx: &mut HashMap<String, Vec<bool>>) -> (Vec<bool>, Vec<bool>) {
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
