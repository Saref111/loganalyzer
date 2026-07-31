# loganalyzer

A command-line tool that parses log files and reports statistics: line
counts, log level breakdown, most frequent messages, and the first/last
timestamps seen.

## Log format

Each line is expected to look like:

```
<timestamp> <LEVEL> <message>
```

For example:

```
2026-07-26T10:00:00 INFO server started
2026-07-26T10:05:00 ERROR disk full
```

Recognized levels are `TRACE`, `DEBUG`, `INFO`, `WARN`, and `ERROR`. Lines
that don't match this shape are counted as "broken" rather than causing the
program to fail.

## Installation

Requires Rust (2024 edition) and Cargo.

```
cargo build --release
```

The binary is written to `target/release/loganalyzer`.

## Usage

```
loganalyzer [OPTIONS] [PATH]
```

Reads from `PATH` if given, otherwise from stdin.

### Options

| Option            | Description                                          | Default |
| ------------------ | ----------------------------------------------------- | ------- |
| `--format <FORMAT>` | Output format: `text` or `json`                      | `text`  |
| `--top <TOP>`       | Number of most frequent messages to include          | `5`     |
| `--level <LEVEL>`   | Only include lines at or above this level             | none    |

Level ordering (low to high): `trace < debug < info < warn < error`.

### Examples

Analyze a file:

```
loganalyzer app.log
```

Read from stdin:

```
cat app.log | loganalyzer
```

Only show `WARN` and above, top 10 messages, as JSON:

```
loganalyzer app.log --level warn --top 10 --format json
```

## Output

The report includes:

- **Lines statistics** — parsed, broken, total, and filtered-out line counts
- **Level statistics** — count of lines per log level
- **Frequency statistics** — the most common messages, up to `--top`
- **First and last timestamps** — earliest and latest timestamps encountered

Example (text format):

```
Lines statistics:

	Parsed lines: 5
	Broken lines: 1
	Total lines: 6
	Filtered out lines: 0

Level statistics:

	INFO: 1
	WARN: 1
	ERROR: 3

Frequency statistics:

	disk full: 2
	high memory: 1
	say "hello": 1
	server started: 1

The first and the last timestamps:

	First timestamp: 2026-07-26T09:00:00
	Last timestamp: 2026-07-26T11:00:00
```
