use clap::{Arg, ArgAction, Command};
use rplt::eval::eval;
use rplt::expr_parser::tokens_to_expr;
use rplt::table::generate_table;
use rplt::token_parser::{token_parser, vars_values};

fn main() {
    let args = Command::new("rplt")
        .version("1.0.0")
        .author("Nowu <nowu.user@gmail.com>").about("RPLT (Rust Propositional Logic Table) is a truth table generator for propositional logic expressions. Written in Rust, it parses logical formulas and outputs their complete truth tables. Supports operators like NOT, AND, OR, CONDITIONAL and BICONDITIONAL")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enable verbose operation")
                .action(ArgAction::SetTrue)
        )
        .subcommand(
            Command::new("table")
                .about("Display a logic propositional table from the input")
                .arg(
                    Arg::new("input")
                        .help("Input string for the table")
                        .required(true)
                        .index(1)
                )
        );
    let matches = args.get_matches();
    let verbose = matches.get_flag("verbose");

    match matches.subcommand() {
        Some(("table", a)) => {
            let i = a.get_one::<String>("input").unwrap();
            let mut tokens = token_parser(&i);
            let mut ctx = vars_values(&tokens);
            let expr = tokens_to_expr(&mut tokens);
            let _ = eval(&expr, &mut ctx);
            println!("{}\n", generate_table(&ctx, verbose));
        }
        _ => {
            eprintln!("Unable to process sub command");
        }
    }
}
