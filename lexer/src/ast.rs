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

fn to_flattened(root: &JsonValue, prefix: Option<String>) -> HashMap<String, String> {
    let mut res = HashMap::new();

    match root {
        JsonValue::Object(items) => {
            for item in items {
                let mut key = item.key.to_owned();
                if let Some(prefix) = &prefix {
                    key.push('.');
                    key.push_str(prefix);
                }
                let value = &item.value;

                match value {
                    JsonValue::Null => {
                        res.insert(key, "null".to_owned());
                    }
                    JsonValue::Boolean(b) => {
                        res.insert(key, b.to_string());
                    }
                    JsonValue::Number(n) => {
                        res.insert(key, n.to_string());
                    }
                    JsonValue::String(s) => {
                        res.insert(key, String::from_utf8_lossy(s).into_owned());
                    }
                    JsonValue::Array(vec) => todo!(),
                    JsonValue::Object(vec) => {
                        let nested = to_flattened(value, Some(key));

                        res.extend(nested);

                        return res;
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
            // JsonItem {
            //     key: "array",
            //     value: JsonValue::Array(vec![
            //         JsonValue::Number(1),
            //         JsonValue::Number(2),
            //         JsonValue::Number(3),
            //         JsonValue::Number(4),
            //         JsonValue::String(b"five"),
            //         JsonValue::Boolean(true),
            //     ]),
            // },
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
