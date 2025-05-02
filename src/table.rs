use crate::Value;
use colored::Colorize;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Cell, CellAlignment, ContentArrangement, Table};
use std::collections::HashMap;

pub fn generate_table(ctx: &HashMap<String, Vec<Value>>) -> Table {
    let mut t = Table::new();
    t.set_content_arrangement(ContentArrangement::Dynamic)
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS);

    let keys = {
        let mut keys = ctx.keys().map(String::as_str).collect::<Vec<&str>>();
        keys.sort_by(|a, b| a.len().cmp(&b.len()).then(a.cmp(b)));
        keys
    };
    t.set_header(&keys);
    let rows_amount = match ctx.values().next() {
        None => 0,
        Some(v) => v.len(),
    };
    for i in 0..rows_amount {
        let row = keys
            .iter()
            .map(|key| match ctx.get(*key) {
                None => panic!("key {} not found", key),
                Some(values) => match values.get(i).unwrap() {
                    Value::Value(p) => {
                        if *p {
                            "V".normal()
                        } else {
                            "F".normal()
                        }
                    }
                    Value::Expr(Some(p), r, None) => {
                        let p = if *p { "V".dimmed() } else { "F".dimmed() };
                        let r = if *r { "V".normal() } else { "F".normal() };
                        format!("{p} [{r}]").normal()
                    }
                    Value::Expr(Some(p), r, Some(q)) => {
                        let p = if *p { "V".dimmed() } else { "F".dimmed() };
                        let r = if *r { "V".normal() } else { "F".normal() };
                        let q = if *q { "V".dimmed() } else { "F".dimmed() };
                        format!("{p} [{r}] {q}").normal()
                    }
                    _ => panic!("what"),
                },
            })
            .map(|x| Cell::new(x).set_alignment(CellAlignment::Center))
            .collect::<Vec<_>>();
        t.add_row(row);
    }
    t
}
