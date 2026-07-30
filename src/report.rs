use crate::stats::{Shape, StatValue, Value};
use std::fmt::Write;

pub enum ReportType {
    Text,
    Json,
}

pub trait Report {
    fn render(&self, stats: &[StatValue]) -> String;
}

struct TextReport;

impl Report for TextReport {
    fn render(&self, stats: &[StatValue]) -> String {
        let mut out = String::new();
        for (section, entries, _) in stats {
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

struct JsonReport;

impl Report for JsonReport {
    fn render(&self, stats: &[StatValue]) -> String {
        let mut out = String::new();
        out.push('{');
        let mut parts = vec![];

        for (section, entries, json_type) in stats {
            let mut section_out = format!("\"{}\": ", escape(section));

            let json_structure = match json_type {
                Shape::Pairs => object(entries),
                Shape::Records => message_array(entries),
            };

            section_out.push_str(&json_structure);

            parts.push(section_out);
        }

        out.push_str(&parts.join(","));
        out.push('}');
        out
    }
}

fn escape(string: &str) -> String {
    string.replace("\\", "\\\\").replace("\"", "\\\"")
}

fn match_key_value_json(key: &str, value: &Value) -> String {
    let value = match value {
        Value::Count(n) => n.to_string(),
        Value::Text(t) => format!("\"{}\"", escape(t)),
        Value::Null => "null".to_string(),
    };

    format!("\"{}\": {}", escape(key), value)
}

fn object(entries: &[(&str, Value)]) -> String {
    let parts: Vec<_> = entries
        .iter()
        .map(|(k, v)| match_key_value_json(k, v))
        .collect();
    format!("{{{}}}", parts.join(","))
}

fn message_array(entries: &[(&str, Value)]) -> String {
    let parts: Vec<_> = entries
        .iter()
        .map(|(msg, count)| {
            format!(
                "{{{},{}}}",
                match_key_value_json("message", &Value::Text(msg)),
                match_key_value_json("count", count)
            )
        })
        .collect();
    format!("[{}]", parts.join(","))
}

pub fn get_report(stats: &[StatValue], report_type: ReportType) -> String {
    let report: Box<dyn Report> = match report_type {
        ReportType::Text => Box::new(TextReport),
        ReportType::Json => Box::new(JsonReport),
    };

    report.render(stats)
}
