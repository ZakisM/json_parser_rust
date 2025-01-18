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

    // pub fn as_flattened(&self) -> BTreeMap<Cow<'a, str>, String> {
    //     let mut res = BTreeMap::new();

    //     match self {
    //         JsonValue::Object(root_properties) => {
    //             for root_prop in root_properties {
    //                 let mut stack = vec![root_prop.clone()];

    //                 while let Some(root_prop) = stack.pop() {
    //                     match root_prop.value {
    //                         JsonValue::Object(object) => {
    //                             for nested_prop in object.into_iter() {
    //                                 let prop = JsonProperty {
    //                                     key: Cow::Owned(format!(
    //                                         "{}.{}",
    //                                         root_prop.key.clone().into_owned(),
    //                                         nested_prop.key
    //                                     )),
    //                                     value: nested_prop.value,
    //                                 };

    //                                 stack.push(prop);
    //                             }
    //                         }
    //                         JsonValue::Array(array) => {
    //                             for (index, nested_value) in array.into_iter().enumerate() {
    //                                 let prop = JsonProperty {
    //                                     key: Cow::Owned(format!(
    //                                         "{}.{:03}",
    //                                         root_prop.key.clone().into_owned(),
    //                                         index
    //                                     )),
    //                                     value: nested_value,
    //                                 };

    //                                 stack.push(prop);
    //                             }
    //                         }
    //                         _ => {
    //                             res.insert(root_prop.key, root_prop.value.inner_value());
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //         JsonValue::Array(array) => {
    //             let mut properties = Vec::new();

    //             for (index, nested_value) in array.iter().enumerate() {
    //                 properties.push(JsonProperty {
    //                     key: Cow::Owned(format!("{:03}", index)),
    //                     value: nested_value.clone(),
    //                 });
    //             }

    //             let root = JsonValue::Object(properties);

    //             return root.as_flattened();
    //         }
    //         _ => panic!("expected root to be of type JsonValue::Object"),
    //     }

    //     res
    // }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn root_object_flattened() {
//         let root = JsonValue::Object(vec![
//             JsonProperty::from(("name", JsonValue::String("John"))),
//             JsonProperty::from(("age", JsonValue::Number(30))),
//             JsonProperty::from(("isStudent", JsonValue::Boolean(false))),
//             JsonProperty::from((
//                 "address",
//                 JsonValue::Object(vec![
//                     JsonProperty::from((
//                         "street",
//                         JsonValue::Object(vec![JsonProperty::from((
//                             "number",
//                             JsonValue::Number(95),
//                         ))]),
//                     )),
//                     JsonProperty::from(("city", JsonValue::String("New York"))),
//                     JsonProperty::from(("zipcode", JsonValue::Null)),
//                 ]),
//             )),
//             JsonProperty::from((
//                 "courses",
//                 JsonValue::Array(vec![
//                     JsonValue::Object(vec![
//                         JsonProperty::from(("courseName", JsonValue::String("Math"))),
//                         JsonProperty::from(("grade", JsonValue::String("A"))),
//                     ]),
//                     JsonValue::Object(vec![
//                         JsonProperty::from(("courseName", JsonValue::String("Science"))),
//                         JsonProperty::from(("grade", JsonValue::String("B"))),
//                     ]),
//                 ]),
//             )),
//             JsonProperty::from((
//                 "preferences",
//                 JsonValue::Object(vec![
//                     JsonProperty::from(("notifications", JsonValue::Boolean(true))),
//                     JsonProperty::from(("theme", JsonValue::String("dark"))),
//                 ]),
//             )),
//             JsonProperty::from((
//                 "scores",
//                 JsonValue::Array(vec![
//                     JsonValue::Number(95),
//                     JsonValue::Number(88),
//                     JsonValue::Number(76),
//                 ]),
//             )),
//             JsonProperty::from((
//                 "metadata",
//                 JsonValue::Object(vec![
//                     JsonProperty::from(("createdAt", JsonValue::String("2023-10-01T12:34:56Z"))),
//                     JsonProperty::from(("updatedAt", JsonValue::String("2023-10-01T12:34:56Z"))),
//                 ]),
//             )),
//         ]);

//         assert_eq!(
//             root.as_flattened(),
//             BTreeMap::from([
//                 ("address.city".into(), "New York".into()),
//                 ("address.street.number".into(), "95".into()),
//                 ("address.zipcode".into(), "null".into()),
//                 ("age".into(), "30".into()),
//                 ("courses.000.courseName".into(), "Math".into()),
//                 ("courses.000.grade".into(), "A".into()),
//                 ("courses.001.courseName".into(), "Science".into()),
//                 ("courses.001.grade".into(), "B".into()),
//                 ("isStudent".into(), "false".into()),
//                 ("metadata.createdAt".into(), "2023-10-01T12:34:56Z".into()),
//                 ("metadata.updatedAt".into(), "2023-10-01T12:34:56Z".into()),
//                 ("name".into(), "John".into()),
//                 ("preferences.notifications".into(), "true".into()),
//                 ("preferences.theme".into(), "dark".into()),
//                 ("scores.000".into(), "95".into()),
//                 ("scores.001".into(), "88".into()),
//                 ("scores.002".into(), "76".into()),
//             ])
//         );
//     }

//     #[test]
//     fn root_array_flattened() {
//         let root = JsonValue::Array(vec![
//             JsonValue::Object(vec![
//                 JsonProperty::from(("albumId", JsonValue::Number(1))),
//                 JsonProperty::from(("id", JsonValue::Number(1))),
//                 JsonProperty::from((
//                     "title",
//                     JsonValue::String("accusamus beatae ad facilis cum similique qui sunt"),
//                 )),
//                 JsonProperty::from((
//                     "url",
//                     JsonValue::String("https://via.placeholder.com/600/92c952"),
//                 )),
//                 JsonProperty::from((
//                     "thumbnailUrl",
//                     JsonValue::String("https://via.placeholder.com/150/92c952"),
//                 )),
//             ]),
//             JsonValue::Object(vec![
//                 JsonProperty::from(("albumId", JsonValue::Number(1))),
//                 JsonProperty::from(("id", JsonValue::Number(2))),
//                 JsonProperty::from((
//                     "title",
//                     JsonValue::String("reprehenderit est deserunt velit ipsam"),
//                 )),
//                 JsonProperty::from((
//                     "url",
//                     JsonValue::String("https://via.placeholder.com/600/771796"),
//                 )),
//                 JsonProperty::from((
//                     "thumbnailUrl",
//                     JsonValue::String("https://via.placeholder.com/150/771796"),
//                 )),
//             ]),
//         ]);

//         assert_eq!(
//             root.as_flattened(),
//             BTreeMap::from([
//                 ("000.albumId".into(), "1".into()),
//                 ("000.id".into(), "1".into()),
//                 (
//                     "000.thumbnailUrl".into(),
//                     "https://via.placeholder.com/150/92c952".into()
//                 ),
//                 (
//                     "000.title".into(),
//                     "accusamus beatae ad facilis cum similique qui sunt".into()
//                 ),
//                 (
//                     "000.url".into(),
//                     "https://via.placeholder.com/600/92c952".into()
//                 ),
//                 ("001.albumId".into(), "1".into()),
//                 ("001.id".into(), "2".into()),
//                 (
//                     "001.thumbnailUrl".into(),
//                     "https://via.placeholder.com/150/771796".into()
//                 ),
//                 (
//                     "001.title".into(),
//                     "reprehenderit est deserunt velit ipsam".into()
//                 ),
//                 (
//                     "001.url".into(),
//                     "https://via.placeholder.com/600/771796".into()
//                 ),
//             ])
//         );
//     }
// }
