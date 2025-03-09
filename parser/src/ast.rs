use bumpalo::collections::Vec;
use std::{borrow::Cow, collections::BTreeMap};

#[derive(Debug, Clone, PartialEq)]
pub struct JsonProperty<'a> {
    pub key: Cow<'a, str>,
    pub value: JsonValue<'a>,
}

impl<'a> From<(&'a str, JsonValue<'a>)> for JsonProperty<'a> {
    fn from(item: (&'a str, JsonValue<'a>)) -> Self {
        Self {
            key: item.0.into(),
            value: item.1,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue<'a> {
    Null,
    Boolean(bool),
    Number(f64),
    String(&'a str),
    Object(Vec<'a, JsonProperty<'a>>),
    Array(Vec<'a, JsonValue<'a>>),
}

impl JsonValue<'_> {
    pub fn flattened(&self) -> BTreeMap<String, String> {
        let mut res = BTreeMap::new();

        self.flatten("", &mut res);

        res
    }

    fn flatten(&self, prefix: &str, res: &mut BTreeMap<String, String>) {
        match self {
            JsonValue::Null => {
                res.insert(prefix.to_owned(), "null".to_string());
            }
            JsonValue::Boolean(val) => {
                res.insert(prefix.to_owned(), val.to_string());
            }
            JsonValue::Number(val) => {
                res.insert(prefix.to_owned(), val.to_string());
            }
            JsonValue::String(val) => {
                res.insert(prefix.to_owned(), val.to_string());
            }
            JsonValue::Object(properties) => {
                for property in properties {
                    let new_prefix = if prefix.is_empty() {
                        property.key.to_string()
                    } else {
                        format!("{}.{}", prefix, property.key)
                    };

                    property.value.flatten(&new_prefix, res);
                }
            }
            JsonValue::Array(json_values) => {
                for (index, value) in json_values.iter().enumerate() {
                    let index = format!("{:03}", index);

                    let new_prefix = if prefix.is_empty() {
                        index
                    } else {
                        format!("{}.{}", prefix, index)
                    };

                    value.flatten(&new_prefix, res);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bumpalo::{Bump, vec};

    #[test]
    fn root_object_flattened() {
        let bump = Bump::new();

        let root = JsonValue::Object(vec![
            in &bump;
            JsonProperty::from(("name", JsonValue::String("John"))),
            JsonProperty::from(("age", JsonValue::Number(30.0))),
            JsonProperty::from(("isStudent", JsonValue::Boolean(false))),
            JsonProperty::from((
                "address",
                JsonValue::Object(vec![
                    in &bump;
                    JsonProperty::from((
                        "street",
                        JsonValue::Object(vec![
                            in &bump;
                            JsonProperty::from((
                                "number",
                                JsonValue::Number(95.0),
                            ))]),
                    )),
                    JsonProperty::from(("city", JsonValue::String("New York"))),
                    JsonProperty::from(("zipcode", JsonValue::Null)),
                ]),
            )),
            JsonProperty::from((
                "courses",
                JsonValue::Array(vec![
                    in &bump;
                    JsonValue::Object(vec![
                        in &bump;
                        JsonProperty::from(("courseName", JsonValue::String("Math"))),
                        JsonProperty::from(("grade", JsonValue::String("A"))),
                    ]),
                    JsonValue::Object(vec![
                        in &bump;
                        JsonProperty::from(("courseName", JsonValue::String("Science"))),
                        JsonProperty::from(("grade", JsonValue::String("B"))),
                    ]),
                ]),
            )),
            JsonProperty::from((
                "preferences",
                JsonValue::Object(vec![
                    in &bump;
                    JsonProperty::from(("notifications", JsonValue::Boolean(true))),
                    JsonProperty::from(("theme", JsonValue::String("dark"))),
                ]),
            )),
            JsonProperty::from((
                "scores",
                JsonValue::Array(vec![
                    in &bump;
                    JsonValue::Number(95.0),
                    JsonValue::Number(88.0),
                    JsonValue::Number(76.0),
                ]),
            )),
            JsonProperty::from((
                "metadata",
                JsonValue::Object(vec![
                    in &bump;
                    JsonProperty::from(("createdAt", JsonValue::String("2023-10-01T12:34:56Z"))),
                    JsonProperty::from(("updatedAt", JsonValue::String("2023-10-01T12:34:56Z"))),
                ]),
            )),
        ]);

        assert_eq!(
            root.flattened(),
            BTreeMap::from([
                ("address.city".into(), "New York".into()),
                ("address.street.number".into(), "95".into()),
                ("address.zipcode".into(), "null".into()),
                ("age".into(), "30".into()),
                ("courses.000.courseName".into(), "Math".into()),
                ("courses.000.grade".into(), "A".into()),
                ("courses.001.courseName".into(), "Science".into()),
                ("courses.001.grade".into(), "B".into()),
                ("isStudent".into(), "false".into()),
                ("metadata.createdAt".into(), "2023-10-01T12:34:56Z".into()),
                ("metadata.updatedAt".into(), "2023-10-01T12:34:56Z".into()),
                ("name".into(), "John".into()),
                ("preferences.notifications".into(), "true".into()),
                ("preferences.theme".into(), "dark".into()),
                ("scores.000".into(), "95".into()),
                ("scores.001".into(), "88".into()),
                ("scores.002".into(), "76".into()),
            ])
        );
    }

    #[test]
    fn root_array_flattened() {
        let bump = Bump::new();

        let root = JsonValue::Array(vec![
            in &bump;
            JsonValue::Object(vec![
                in &bump;
                JsonProperty::from(("albumId", JsonValue::Number(1.0))),
                JsonProperty::from(("id", JsonValue::Number(1.0))),
                JsonProperty::from((
                    "title",
                    JsonValue::String("accusamus beatae ad facilis cum similique qui sunt"),
                )),
                JsonProperty::from((
                    "url",
                    JsonValue::String("https://via.placeholder.com/600/92c952"),
                )),
                JsonProperty::from((
                    "thumbnailUrl",
                    JsonValue::String("https://via.placeholder.com/150/92c952"),
                )),
            ]),
            JsonValue::Object(vec![
                in &bump;
                JsonProperty::from(("albumId", JsonValue::Number(1.0))),
                JsonProperty::from(("id", JsonValue::Number(2.0))),
                JsonProperty::from((
                    "title",
                    JsonValue::String("reprehenderit est deserunt velit ipsam"),
                )),
                JsonProperty::from((
                    "url",
                    JsonValue::String("https://via.placeholder.com/600/771796"),
                )),
                JsonProperty::from((
                    "thumbnailUrl",
                    JsonValue::String("https://via.placeholder.com/150/771796"),
                )),
            ]),
        ]);

        assert_eq!(
            root.flattened(),
            BTreeMap::from([
                ("000.albumId".into(), "1".into()),
                ("000.id".into(), "1".into()),
                (
                    "000.thumbnailUrl".into(),
                    "https://via.placeholder.com/150/92c952".into()
                ),
                (
                    "000.title".into(),
                    "accusamus beatae ad facilis cum similique qui sunt".into()
                ),
                (
                    "000.url".into(),
                    "https://via.placeholder.com/600/92c952".into()
                ),
                ("001.albumId".into(), "1".into()),
                ("001.id".into(), "2".into()),
                (
                    "001.thumbnailUrl".into(),
                    "https://via.placeholder.com/150/771796".into()
                ),
                (
                    "001.title".into(),
                    "reprehenderit est deserunt velit ipsam".into()
                ),
                (
                    "001.url".into(),
                    "https://via.placeholder.com/600/771796".into()
                ),
            ])
        );
    }
}
