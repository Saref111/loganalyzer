use std::fmt::Display;

use regex::Regex;

use crate::levels::Level;

pub struct LogLine<'a> {
    timestamp: &'a str,
    level: Level,
    message: &'a str,
}

impl<'a> LogLine<'a> {
    fn new(timestamp: &'a str, level: Level, message: &'a str) -> Self {
        Self {
            timestamp,
            level,
            message,
        }
    }

    pub fn level(&self) -> Level {
        self.level
    }

    pub fn timestamp(&self) -> &'a str {
        self.timestamp
    }

    pub fn message(&self) -> &'a str {
        self.message
    }
}

impl Display for LogLine<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Timestamp: {}\nLevel: {}\nMessage: {}\n",
            self.timestamp,
            self.level.to_str(),
            self.message
        )
    }
}

pub struct LogParser {
    re: Regex,
}

impl LogParser {
    pub fn new() -> Self {
        Self {
            re: Regex::new(r"^(?P<timestamp>\S+)\s+(?P<level>\S+)\s+(?P<message>.*)$")
                .expect("Wrong regex in LogParser."),
        }
    }

    pub fn parse<'a>(&self, line: &'a str) -> Option<LogLine<'a>> {
        let parsed = self.re.captures(line)?;
        let (_, [timestamp, level, message]) = parsed.extract();
        Some(LogLine::new(timestamp, Level::from_str(level)?, message))
    }
}
