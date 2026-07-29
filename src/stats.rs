// топ-N найчастіших повідомлень (згадай день 1 і день 3.5 — тай-брейк за алфавітом при рівній частоті)
// часовий діапазон: перша й остання мітка часу

use std::collections::HashMap;

use crate::{levels::Level, parse::LogLine};

trait Stat {
    fn name(&self) -> &'static str;
    fn update(&mut self, log_line: Option<LogLine>);
    fn value(&self) -> String;
    fn report(&self) -> String {
        format!("{}: {}", self.name(), self.value())
    }
}

struct LinesStat {
    total: u64,
    parsed: u64,
    broken: u64,
}

impl Stat for LinesStat {
    fn name(&self) -> &'static str {
        "Lines statistics"
    }

    fn update(&mut self, log_line: Option<LogLine>) {
        self.total += 1;
        match log_line {
            Some(_) => self.parsed += 1,
            None => self.broken += 1,
        }
    }

    fn value(&self) -> String {
        format!(
            "\nParsed lines: {}\n Broken lines: {}\n Total lines: {}",
            self.parsed, self.broken, self.total
        )
    }
}

struct LevelStat {
    map: HashMap<Level, u64>,
}

impl Stat for LevelStat {
    fn name(&self) -> &'static str {
        "Level statistics"
    }

    fn update(&mut self, log_line: Option<LogLine>) {
        if let Some(log_line) = log_line {
            self.map
                .entry(log_line.level())
                .and_modify(|v| *v += 1)
                .or_insert(1);
        }
    }

    fn value(&self) -> String {
        let entries = self.map.iter().collect::<Vec<_>>();
        let mut value = String::from("\n");

        for (level, amount) in entries {
            value.push_str(&format!("{}: {}\n", level.to_str(), amount));
        }

        value
    }
}
