use std::collections::HashMap;

use crate::{
    levels::Level,
    parse::{LogLine, LogParser},
};

type StatName<'a> = &'a str;
type Key<'a> = &'a str;

pub enum Value<'a> {
    Count(u64),
    Text(&'a str),
    Null,
}

pub type StatValue<'a> = (StatName<'a>, Vec<(Key<'a>, Value<'a>)>);

trait Stat<'a> {
    fn name(&self) -> &'static str;
    fn update(&mut self, log_line: &LogLine<'a>);
    fn value(&self) -> StatValue<'a>;
}

#[derive(Debug, Default)]
struct LevelStat {
    map: HashMap<Level, u64>,
}

impl<'a> Stat<'a> for LevelStat {
    fn name(&self) -> &'static str {
        "Level statistics"
    }

    fn update(&mut self, log_line: &LogLine) {
        self.map
            .entry(log_line.level())
            .and_modify(|v| *v += 1)
            .or_insert(1);
    }

    fn value(&self) -> StatValue<'a> {
        (
            self.name(),
            self.map
                .iter()
                .map(|(l, a)| (l.to_str(), Value::Count(*a)))
                .collect(),
        )
    }
}

#[derive(Debug)]
struct FrequencyStat<'a> {
    map: HashMap<&'a str, u64>,
    capacity: usize,
}

impl FrequencyStat<'_> {
    fn new(capacity: Option<usize>) -> Self {
        match capacity {
            Some(capacity) => Self {
                map: HashMap::new(),
                capacity,
            },
            None => FrequencyStat::default(),
        }
    }
}

impl Default for FrequencyStat<'_> {
    fn default() -> Self {
        Self {
            map: HashMap::new(),
            capacity: 5,
        }
    }
}

impl<'a> Stat<'a> for FrequencyStat<'a> {
    fn name(&self) -> &'static str {
        "Frequency statistics"
    }

    fn update(&mut self, log_line: &LogLine<'a>) {
        self.map
            .entry(log_line.message())
            .and_modify(|v| *v += 1)
            .or_insert(1);
    }

    fn value(&self) -> StatValue<'a> {
        let mut vec = self
            .map
            .iter()
            .map(|(msg, amount)| (*msg, *amount))
            .collect::<Vec<_>>();
        vec.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(b.0)));

        let mut vec = vec
            .into_iter()
            .map(|(msg, amount)| (msg, Value::Count(amount)))
            .collect::<Vec<_>>();
        vec.truncate(self.capacity);

        (self.name(), vec)
    }
}

#[derive(Debug, Default)]
struct TimeStat<'a> {
    first: Option<&'a str>,
    last: Option<&'a str>,
}

impl<'a> Stat<'a> for TimeStat<'a> {
    fn name(&self) -> &'static str {
        "The first and the last timestamps"
    }

    fn update(&mut self, log_line: &LogLine<'a>) {
        let current_ts = log_line.timestamp();
        self.first = Some(self.first.map_or(current_ts, |f| f.min(current_ts)));
        self.last = Some(self.last.map_or(current_ts, |l| l.max(current_ts)));
    }

    fn value(&self) -> StatValue<'a> {
        let first = self.first.map_or(Value::Null, Value::Text);
        let last = self.last.map_or(Value::Null, Value::Text);
        (
            self.name(),
            vec![("First timestamp", first), ("Last timestamp", last)],
        )
    }
}

pub fn get_stats_values<'a>(
    lines: impl Iterator<Item = &'a str>,
    parser: &LogParser,
    top_n: Option<usize>,
) -> Vec<StatValue<'a>> {
    let mut total: u64 = 0;
    let mut parsed: u64 = 0;
    let mut broken: u64 = 0;

    let mut stats: Vec<Box<dyn Stat>> = vec![
        Box::new(LevelStat::default()),
        Box::new(FrequencyStat::new(top_n)),
        Box::new(TimeStat::default()),
    ];

    for line in lines {
        match parser.parse(line) {
            Some(log_line) => {
                stats.iter_mut().for_each(|s| s.update(&log_line));
                parsed += 1;
            }
            None => broken += 1,
        }
        total += 1;
    }

    let mut stats_values: Vec<StatValue> = vec![(
        "Lines statistics",
        vec![
            ("Parsed lines", Value::Count(parsed)),
            ("Broken lines", Value::Count(broken)),
            ("Total lines", Value::Count(total)),
        ],
    )];
    stats_values.extend(stats.iter().map(|s| s.value()));

    stats_values
}
