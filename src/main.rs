use rplt::eval::eval;
use rplt::expr_parser::tokens_to_expr;
use rplt::table::generate_table;
use rplt::token_parser::{parser, vars_values};

fn main() {
    let s = vec![
        "p -> ( p or q )",
        "( p and q ) and ~ p",
        "( p or q ) -> q",
        "( ( p or q ) and ( ~ r or s ) ) -> s",
    ];
    for i in s {
        let mut tokens = parser(i);
        let mut ctx = vars_values(&tokens);
        let expr = tokens_to_expr(&mut tokens);
        let _ = eval(&expr, &mut ctx);
        println!("{}\n", generate_table(&ctx));
    }
}
