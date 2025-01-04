use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq)]
pub struct JsonProperty<'a> {
    pub key: &'a str,
    pub value: JsonValue<'a>,
}

impl<'a> From<(&'a str, JsonValue<'a>)> for JsonProperty<'a> {
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
    Object(Vec<JsonProperty<'a>>),
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
            JsonProperty::from(("name", JsonValue::String("John"))),
            JsonProperty::from(("age", JsonValue::Number(30))),
            JsonProperty::from(("isStudent", JsonValue::Boolean(false))),
            JsonProperty::from((
                "address",
                JsonValue::Object(vec![
                    JsonProperty::from(("street", JsonValue::String("123 Main St"))),
                    JsonProperty::from(("city", JsonValue::String("New York"))),
                    JsonProperty::from(("zipcode", JsonValue::Null)),
                ]),
            )),
            JsonProperty::from((
                "courses",
                JsonValue::Array(vec![
                    JsonValue::Object(vec![
                        JsonProperty::from(("courseName", JsonValue::String("Math"))),
                        JsonProperty::from(("grade", JsonValue::String("A"))),
                    ]),
                    JsonValue::Object(vec![
                        JsonProperty::from(("courseName", JsonValue::String("Science"))),
                        JsonProperty::from(("grade", JsonValue::String("B"))),
                    ]),
                ]),
            )),
            JsonProperty::from((
                "preferences",
                JsonValue::Object(vec![
                    JsonProperty::from(("notifications", JsonValue::Boolean(true))),
                    JsonProperty::from(("theme", JsonValue::String("dark"))),
                ]),
            )),
            JsonProperty::from((
                "scores",
                JsonValue::Array(vec![
                    JsonValue::Number(95),
                    JsonValue::Number(88),
                    JsonValue::Number(76),
                ]),
            )),
            JsonProperty::from((
                "metadata",
                JsonValue::Object(vec![
                    JsonProperty::from(("createdAt", JsonValue::String("2023-10-01T12:34:56Z"))),
                    JsonProperty::from(("updatedAt", JsonValue::String("2023-10-01T12:34:56Z"))),
                ]),
            )),
        ]);

        dbg!(&to_flattened(&root, None));
    }
}
