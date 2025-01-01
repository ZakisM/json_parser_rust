use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct JsonItem<'a> {
    key: &'a str,
    value: JsonValue<'a>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum JsonValue<'a> {
    Null,
    Boolean(bool),
    Number(usize),
    String(&'a [u8]),
    Array(Vec<JsonValue<'a>>),
    Object(Vec<JsonItem<'a>>),
}

impl<'a> JsonValue<'a> {
    fn inner_value(&self) -> String {
        match self {
            JsonValue::Null => "null".to_owned(),
            JsonValue::Boolean(b) => b.to_string(),
            JsonValue::Number(n) => n.to_string(),
            JsonValue::String(s) => String::from_utf8_lossy(s).into_owned(),
            JsonValue::Array(_) => "Array".to_string(),
            JsonValue::Object(_) => "Object".to_string(),
        }
    }
}

fn to_flattened(root: &JsonValue, prefix: Option<String>) -> HashMap<String, String> {
    let mut res = HashMap::new();

    match root {
        JsonValue::Object(entries) => {
            for item in entries {
                let key = prefix
                    .as_ref()
                    .map_or_else(|| item.key.to_owned(), |pre| format!("{pre}.{}", item.key));

                let value = &item.value;

                match value {
                    JsonValue::Array(array) => {
                        for (index, arr_item) in array.iter().enumerate() {
                            match arr_item {
                                JsonValue::Array(vec) => todo!(),
                                JsonValue::Object(vec) => todo!(),
                                _ => {
                                    res.insert(
                                        format!("{key}.{:03}", index),
                                        arr_item.inner_value(),
                                    );
                                }
                            }
                        }
                    }
                    JsonValue::Object(vec) => {
                        let nested = to_flattened(value, Some(key));

                        res.extend(nested);

                        return res;
                    }
                    _ => {
                        res.insert(key, value.inner_value());
                    }
                };
            }
        }
        _ => panic!("must be called with a JsonValue::Object"),
    };

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flattened() {
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
            JsonItem {
                key: "anotherNestedObject",
                value: JsonValue::Object(vec![JsonItem {
                    key: "level1",
                    value: JsonValue::Object(vec![JsonItem {
                        key: "key",
                        value: JsonValue::String(b"value"),
                    }]),
                }]),
            },
        ]);

        dbg!(&to_flattened(&root, None));
    }
}
