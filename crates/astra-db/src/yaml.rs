//! Safe-subset YAML reader (`BUILD_SPEC.md` §2): one document, no anchors,
//! aliases, tags or duplicate keys, converted to a `serde_json::Value`.

use std::borrow::Cow;

use saphyr_parser::{Event, Parser, ScalarStyle, Span, StrInput};
use serde_json::{Map, Number, Value};

/// One parser event with its position.
type Item<'input> = (Event<'input>, Span);

/// Event-level reader over a single YAML string.
struct Reader<'input> {
    /// Underlying event parser.
    parser: Parser<'input, StrInput<'input>>,
}

/// Rejects anchors (`&a`) and tags (`!t`) on a node.
fn check_node(anchor: usize, tagged: bool, line: usize) -> Result<(), String> {
    if anchor != 0 {
        return Err(format!("line {line} : anchor not allowed"));
    }
    if tagged {
        return Err(format!("line {line} : tag not allowed"));
    }
    Ok(())
}

/// Resolves a plain scalar with the YAML 1.2 core schema (null, booleans, integers).
fn plain(text: &str) -> Value {
    match text {
        "" | "~" | "null" | "Null" | "NULL" => Value::Null,
        "true" | "True" | "TRUE" => Value::Bool(true),
        "false" | "False" | "FALSE" => Value::Bool(false),
        _ => text
            .parse::<i64>()
            .map_or_else(|_err| Value::String(text.to_owned()), |num| Value::Number(Number::from(num))),
    }
}

/// Converts a scalar; block scalars lose only their final newline (§5).
fn scalar(text: Cow<'_, str>, style: ScalarStyle) -> Value {
    match style {
        ScalarStyle::Plain => plain(&text),
        ScalarStyle::Literal | ScalarStyle::Folded => {
            Value::String(text.strip_suffix('\n').unwrap_or(&text).to_owned())
        }
        ScalarStyle::SingleQuoted | ScalarStyle::DoubleQuoted => Value::String(text.into_owned()),
    }
}

impl<'input> Reader<'input> {
    /// Next event, with scanner errors turned into messages.
    fn next(&mut self) -> Result<Item<'input>, String> {
        match self.parser.next_event() {
            Some(Ok(item)) => Ok(item),
            Some(Err(err)) => Err(format!("line {}: {}", err.marker().line(), err.info())),
            None => Err("unexpected end of file".to_owned()),
        }
    }

    /// Converts the node starting with `item`.
    fn node(&mut self, item: Item<'input>) -> Result<Value, String> {
        let (event, span) = item;
        let line = span.start.line();
        match event {
            Event::Scalar(text, style, anchor, tag) => {
                check_node(anchor, tag.is_some(), line)?;
                Ok(scalar(text, style))
            }
            Event::SequenceStart(anchor, tag) => {
                check_node(anchor, tag.is_some(), line)?;
                self.sequence()
            }
            Event::MappingStart(anchor, tag) => {
                check_node(anchor, tag.is_some(), line)?;
                self.mapping()
            }
            Event::Alias(_) => Err(format!("line {line} : alias not allowed")),
            Event::Nothing
            | Event::StreamStart
            | Event::StreamEnd
            | Event::DocumentStart(_)
            | Event::DocumentEnd
            | Event::SequenceEnd
            | Event::MappingEnd => Err(format!("line {line} : unexpected structure")),
        }
    }

    /// Reads sequence items up to `SequenceEnd`.
    fn sequence(&mut self) -> Result<Value, String> {
        let mut items = Vec::new();
        loop {
            let item = self.next()?;
            if matches!(item.0, Event::SequenceEnd) {
                return Ok(Value::Array(items));
            }
            items.push(self.node(item)?);
        }
    }

    /// Reads mapping entries up to `MappingEnd`; keys must be unique scalars.
    fn mapping(&mut self) -> Result<Value, String> {
        let mut map = Map::new();
        loop {
            let (event, span) = self.next()?;
            let line = span.start.line();
            if matches!(event, Event::MappingEnd) {
                return Ok(Value::Object(map));
            }
            let Event::Scalar(key, _, anchor, tag) = event else {
                return Err(format!("line {line} : non-scalar key"));
            };
            check_node(anchor, tag.is_some(), line)?;
            if map.contains_key(key.as_ref()) {
                return Err(format!("line {line} : duplicate key {key}"));
            }
            let item = self.next()?;
            let value = self.node(item)?;
            drop(map.insert(key.into_owned(), value));
        }
    }

    /// Consumes `StreamStart` and `DocumentStart`.
    fn start(&mut self) -> Result<(), String> {
        let (first, _) = self.next()?;
        if !matches!(first, Event::StreamStart) {
            return Err("stream start expected".to_owned());
        }
        let (second, _) = self.next()?;
        if matches!(second, Event::DocumentStart(_)) {
            return Ok(());
        }
        Err("empty file".to_owned())
    }

    /// Consumes `DocumentEnd` and `StreamEnd`; a second document is an error.
    fn end(&mut self) -> Result<(), String> {
        let (doc_end, _) = self.next()?;
        let (stream_end, span) = self.next()?;
        if matches!(doc_end, Event::DocumentEnd) && matches!(stream_end, Event::StreamEnd) {
            return Ok(());
        }
        Err(format!("line {} : only one YAML document per file", span.start.line()))
    }
}

/// Parses one YAML document of the safe subset.
///
/// # Errors
///
/// Returns a message (with line number when known) on any syntax error,
/// anchor, alias, tag, duplicate key, empty file or multiple documents.
pub fn parse(source: &str) -> Result<Value, String> {
    let mut reader = Reader { parser: Parser::new_from_str(source) };
    reader.start()?;
    let root = reader.next()?;
    let value = reader.node(root)?;
    reader.end()?;
    Ok(value)
}
