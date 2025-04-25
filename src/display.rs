use std::fmt::{Display, Result};
use crate::Expr;

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            match self {
                Expr::Var(p) => p.to_string(),
                Expr::Not(p) => format!("~ {p}"),
                Expr::Or(p, q) => format!("({p} or {q})"),
                Expr::And(p, q) => format!("({p} and {q})"),
                Expr::Conditional(p, q) => format!("({p} -> {q})"),
                Expr::BiConditional(p, q) => format!("({p} <-> {q})"),
            }
        )
    }
}
