use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq)]
pub struct JsonProperty {
    pub key: String,
    pub value: JsonValue,
}

impl From<(String, JsonValue)> for JsonProperty {
    fn from(item: (String, JsonValue)) -> Self {
        Self {
            key: item.0,
            value: item.1,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum JsonValue {
    Null,
    Boolean(bool),
    Number(usize),
    String(String),
    Object(Vec<JsonProperty>),
    Array(Vec<JsonValue>),
}

impl JsonValue {
    fn inner_value(&self) -> String {
        match self {
            JsonValue::Null => "null".to_owned(),
            JsonValue::Boolean(b) => b.to_string(),
            JsonValue::Number(n) => n.to_string(),
            JsonValue::String(s) => s.to_owned(),
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
            JsonProperty::from(("name".to_owned(), JsonValue::String("John".to_owned()))),
            JsonProperty::from(("age".to_owned(), JsonValue::Number(30))),
            JsonProperty::from(("isStudent".to_owned(), JsonValue::Boolean(false))),
            JsonProperty::from((
                "address".to_owned(),
                JsonValue::Object(vec![
                    JsonProperty::from((
                        "street".to_owned(),
                        JsonValue::String("123 Main St".to_owned()),
                    )),
                    JsonProperty::from((
                        "city".to_owned(),
                        JsonValue::String("New York".to_owned()),
                    )),
                    JsonProperty::from(("zipcode".to_owned(), JsonValue::Null)),
                ]),
            )),
            JsonProperty::from((
                "courses".to_owned(),
                JsonValue::Array(vec![
                    JsonValue::Object(vec![
                        JsonProperty::from((
                            "courseName".to_owned(),
                            JsonValue::String("Math".to_owned()),
                        )),
                        JsonProperty::from(("grade".to_owned(), JsonValue::String("A".to_owned()))),
                    ]),
                    JsonValue::Object(vec![
                        JsonProperty::from((
                            "courseName".to_owned(),
                            JsonValue::String("Science".to_owned()),
                        )),
                        JsonProperty::from(("grade".to_owned(), JsonValue::String("B".to_owned()))),
                    ]),
                ]),
            )),
            JsonProperty::from((
                "preferences".to_owned(),
                JsonValue::Object(vec![
                    JsonProperty::from(("notifications".to_owned(), JsonValue::Boolean(true))),
                    JsonProperty::from(("theme".to_owned(), JsonValue::String("dark".to_owned()))),
                ]),
            )),
            JsonProperty::from((
                "scores".to_owned(),
                JsonValue::Array(vec![
                    JsonValue::Number(95),
                    JsonValue::Number(88),
                    JsonValue::Number(76),
                ]),
            )),
            JsonProperty::from((
                "metadata".to_owned(),
                JsonValue::Object(vec![
                    JsonProperty::from((
                        "createdAt".to_owned(),
                        JsonValue::String("2023-10-01T12:34:56Z".to_owned()),
                    )),
                    JsonProperty::from((
                        "updatedAt".to_owned(),
                        JsonValue::String("2023-10-01T12:34:56Z".to_owned()),
                    )),
                ]),
            )),
        ]);

        dbg!(&to_flattened(&root, None));
    }
}
