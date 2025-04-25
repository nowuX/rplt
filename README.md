# RLPT
RPLT (Rust Propsoitional Logic Table) is a lightweight truth table generator for propositional logic expressions. Written in Rust, it parses logical formulas and outputs their complete truth tables. Supports operators like NOT, AND, OR, CONDITIONAL and BICONDITIONAL

# Build From Source

To build this project from source, follow these steps:

Install [Rust](https://www.rust-lang.org/tools/install).

### Building the Project
1. Clone this repository to your local machine:
```bash
git clone https://github.com/nowuX/rlpt.git
cd rlpt
cargo run
```

## Usage
Change the `s` value in `main.rs`
```rust
// in main.rs
let s = vec![
    "p -> ( p or q )",
    "( p and q ) and ~ p",
];
// cargo run # in console
/**
Output:
╭───┬───┬──────────┬─────────────────╮
│ p ┆ q ┆ (p or q) ┆ (p -> (p or q)) │
╞═══╪═══╪══════════╪═════════════════╡
│ V ┆ V ┆ V        ┆ V               │
├╌╌╌┼╌╌╌┼╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ V ┆ F ┆ V        ┆ V               │
├╌╌╌┼╌╌╌┼╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ F ┆ V ┆ V        ┆ V               │
├╌╌╌┼╌╌╌┼╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ F ┆ F ┆ F        ┆ V               │
╰───┴───┴──────────┴─────────────────╯

╭───┬───┬─────┬───────────┬─────────────────────╮
│ p ┆ q ┆ ~ p ┆ (p and q) ┆ ((p and q) and ~ p) │
╞═══╪═══╪═════╪═══════════╪═════════════════════╡
│ V ┆ V ┆ F   ┆ V         ┆ F                   │
├╌╌╌┼╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ V ┆ F ┆ F   ┆ F         ┆ F                   │
├╌╌╌┼╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ F ┆ V ┆ V   ┆ F         ┆ F                   │
├╌╌╌┼╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ F ┆ F ┆ V   ┆ F         ┆ F                   │
╰───┴───┴─────┴───────────┴─────────────────────╯
*/
```

## TODO
- [ ] Support arguments with Clap
- [ ] Better display value `~ p` -> `[F] V`, `p or q` -> `V [V] F`
- [ ] Optional better display
- [ ] Just vars and result table
