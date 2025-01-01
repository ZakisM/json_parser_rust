#[derive(Debug, PartialEq, Eq)]
pub enum JsonValue<'a> {
    Null,
    Boolean(bool),
    Number(usize),
    String(&'a [u8]),
    Array(Vec<JsonValue<'a>>),
    Object(Vec<JsonItem<'a>>),
}

#[derive(Debug, PartialEq, Eq)]
pub struct JsonItem<'a> {
    key: &'a str,
    value: JsonValue<'a>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn walk() {
        let root = JsonValue::Object(vec![
            JsonItem {
                key: "string",
                value: JsonValue::String(b"Hello, world!"),
            },
            JsonItem {
                key: "number",
                value: JsonValue::Number(42),
            },
            JsonItem {
                key: "boolean",
                value: JsonValue::Boolean(true),
            },
            JsonItem {
                key: "null",
                value: JsonValue::Null,
            },
            JsonItem {
                key: "array",
                value: JsonValue::Array(vec![
                    JsonValue::Number(1),
                    JsonValue::Number(2),
                    JsonValue::Number(3),
                    JsonValue::Number(4),
                    JsonValue::String(b"five"),
                    JsonValue::Boolean(true),
                ]),
            },
        ]);
    }
}
