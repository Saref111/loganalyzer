use crate::stats::{StatValue, Value};
use std::fmt::Write;

pub trait Report {
    fn render(&self, stats: &[StatValue]) -> String;
}

struct TextReport;

impl Report for TextReport {
    fn render(&self, stats: &[StatValue]) -> String {
        let mut out = String::new();
        for (section, entries) in stats {
            writeln!(out, "{section}:\n").unwrap();
            for (key, val) in entries {
                let value = match val {
                    Value::Count(n) => n.to_string(),
                    Value::Text(t) => t.to_string(),
                    Value::Null => "not found".to_string(),
                };
                writeln!(out, "\t{key}: {value}").unwrap();
            }
            out.push('\n');
        }
        out
    }
}
