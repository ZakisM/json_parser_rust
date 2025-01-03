use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq)]
pub struct JsonItem<'a> {
    pub key: &'a str,
    pub value: JsonValue<'a>,
}

impl<'a> From<(&'a str, JsonValue<'a>)> for JsonItem<'a> {
    fn from(item: (&'a str, JsonValue<'a>)) -> Self {
        Self {
            key: item.0,
            value: item.1,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum JsonValue<'a> {
    Null,
    Boolean(bool),
    Number(usize),
    String(&'a str),
    Object(Vec<JsonItem<'a>>),
    Array(Vec<JsonValue<'a>>),
}

impl<'a> JsonValue<'a> {
    fn inner_value(&self) -> String {
        match self {
            JsonValue::Null => "null".to_owned(),
            JsonValue::Boolean(b) => b.to_string(),
            JsonValue::Number(n) => n.to_string(),
            JsonValue::String(s) => s.to_string(),
            JsonValue::Array(_) => "Array".to_string(),
            JsonValue::Object(_) => "Object".to_string(),
        }
    }
}

fn to_flattened(root: &JsonValue, prefix: Option<String>) -> BTreeMap<String, String> {
    let mut res = BTreeMap::new();

    match root {
        JsonValue::Object(entries) => {
            for item in entries {
                let key = prefix
                    .as_ref()
                    .map_or_else(|| item.key.to_owned(), |pre| format!("{pre}.{}", item.key));

                res.extend(to_flattened(&item.value, Some(key)));
            }
        }
        JsonValue::Array(array) => {
            let prefix = prefix.expect("prefix must be present");

            for (index, item) in array.iter().enumerate() {
                let key = format!("{prefix}.{:03}", index);

                res.extend(to_flattened(item, Some(key)));
            }
        }
        _ => {
            let prefix = prefix.expect("prefix must be present");

            res.insert(prefix, root.inner_value());
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flattened() {
        let root = JsonValue::Object(vec![
            JsonItem {
                key: "name",
                value: JsonValue::String("John"),
            },
            JsonItem {
                key: "age",
                value: JsonValue::Number(30),
            },
            JsonItem {
                key: "isStudent",
                value: JsonValue::Boolean(false),
            },
            JsonItem {
                key: "address",
                value: JsonValue::Object(vec![
                    JsonItem {
                        key: "street",
                        value: JsonValue::String("123 Main St"),
                    },
                    JsonItem {
                        key: "city",
                        value: JsonValue::String("New York"),
                    },
                    JsonItem {
                        key: "zipcode",
                        value: JsonValue::Null,
                    },
                ]),
            },
            JsonItem {
                key: "courses",
                value: JsonValue::Array(vec![
                    JsonValue::Object(vec![
                        JsonItem {
                            key: "courseName",
                            value: JsonValue::String("Math"),
                        },
                        JsonItem {
                            key: "grade",
                            value: JsonValue::String("A"),
                        },
                    ]),
                    JsonValue::Object(vec![
                        JsonItem {
                            key: "courseName",
                            value: JsonValue::String("Science"),
                        },
                        JsonItem {
                            key: "grade",
                            value: JsonValue::String("B"),
                        },
                    ]),
                ]),
            },
            JsonItem {
                key: "preferences",
                value: JsonValue::Object(vec![
                    JsonItem {
                        key: "notifications",
                        value: JsonValue::Boolean(true),
                    },
                    JsonItem {
                        key: "theme",
                        value: JsonValue::String("dark"),
                    },
                ]),
            },
            JsonItem {
                key: "scores",
                value: JsonValue::Array(vec![
                    JsonValue::Number(95),
                    JsonValue::Number(88),
                    JsonValue::Number(76),
                ]),
            },
            JsonItem {
                key: "metadata",
                value: JsonValue::Object(vec![
                    JsonItem {
                        key: "createdAt",
                        value: JsonValue::String("2023-10-01T12:34:56Z"),
                    },
                    JsonItem {
                        key: "updatedAt",
                        value: JsonValue::String("2023-10-01T12:34:56Z"),
                    },
                ]),
            },
        ]);

        dbg!(&to_flattened(&root, None));
    }
}
