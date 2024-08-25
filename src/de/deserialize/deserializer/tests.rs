use super::super::super::parse::parse_with_reader;
use super::*;
use crate::data::mark::Mark;
use serde::Deserialize;
use std::{collections::HashMap, result::Result};

#[test]
fn test_number() {
    let data = parse_with_reader("10").unwrap();
    let result = i32::deserialize(data.deserializer());
    assert_eq!(result, Ok(10));

    let data = parse_with_reader("hello").unwrap();
    let result = i32::deserialize(data.deserializer());
    assert_eq!(
        result,
        Err(marked::DeserializeError::new_invalid_value(
            Mark::new(0, 0),
            "integer in the range from -2^31 to 2^31 - 1".into(),
            None
        ))
    );
}

#[test]
fn test_char() {
    let data = parse_with_reader("> h").unwrap();
    let result = char::deserialize(data.deserializer());
    assert_eq!(result, Ok('h'));

    let data = parse_with_reader("> hello").unwrap();
    let result = char::deserialize(data.deserializer());
    assert_eq!(
        result,
        Err(marked::DeserializeError::new_invalid_value(
            Mark::new(0, 0),
            "one-character string".into(),
            None
        ))
    );
}

#[test]
fn test_str() {
    let data = parse_with_reader("> hello").unwrap();
    let result = <&str>::deserialize(data.deserializer());
    assert_eq!(result, Ok("hello"));

    let data = parse_with_reader("hello").unwrap();
    let result = <&str>::deserialize(data.deserializer());
    assert_eq!(
        result,
        Err(marked::DeserializeError::new_invalid_type(
            Mark::new(0, 0),
            NodeType::Raw,
            &[
                NodeType::String,
                NodeType::Tagged,
                NodeType::Anchor,
                NodeType::Document
            ] as &[_]
        ))
    );
}

#[test]
fn test_option() {
    let data = parse_with_reader("= Some: 42").unwrap();
    let result = Option::<u8>::deserialize(data.deserializer());
    assert_eq!(result, Ok(Some(42)));

    let data = parse_with_reader("Some").unwrap();
    let result = Option::<u8>::deserialize(data.deserializer());
    assert_eq!(
        result,
        Err(marked::DeserializeError::new_invalid_value(
            Mark::new(0, 0),
            "optional value".into(),
            Some(Box::new(marked::DeserializeError::new_invalid_type(
                Mark::new(0, 0),
                NodeType::Raw,
                &[NodeType::Tagged, NodeType::Anchor, NodeType::Document] as &[_]
            )))
        ))
    );
}

#[test]
fn test_seq() {
    let data = parse_with_reader("[0, 2, 67]").unwrap();
    let result = Vec::<u8>::deserialize(data.deserializer());
    assert_eq!(result, Ok(vec![0, 2, 67]));

    let data = parse_with_reader("[0, 2, 457]").unwrap();
    let result = Vec::<u8>::deserialize(data.deserializer());
    assert_eq!(
        result,
        Err(marked::DeserializeError::new_invalid_value(
            Mark::new(0, 7),
            "integer in the range from 0 to 2^8 - 1".into(),
            None
        ))
    );
}

#[test]
fn test_tuple() {
    let data = parse_with_reader("[2, 67]").unwrap();
    let result = <(u8, u8)>::deserialize(data.deserializer());
    assert_eq!(result, Ok((2, 67)));

    let data = parse_with_reader("[0, 2, 457]").unwrap();
    let result = <(u8, u8)>::deserialize(data.deserializer());
    assert_eq!(
        result,
        Err(marked::DeserializeError::new_invalid_length(
            Mark::new(0, 0),
            3,
            Some(Origin::List),
            Some(2),
        ))
    );
}

#[test]
fn test_map() {
    let data = parse_with_reader("first: 42\nsecond: 15").unwrap();
    let result = HashMap::<String, i32>::deserialize(data.deserializer());
    assert_eq!(
        result,
        Ok(HashMap::from([("first".into(), 42), ("second".into(), 15)]))
    );

    let data = parse_with_reader(r#"[["first", 42], ["second"]]"#).unwrap();
    let result = HashMap::<String, i32>::deserialize(data.deserializer());
    assert_eq!(
        result,
        Err(marked::DeserializeError::new_invalid_length(
            Mark::new(0, 16),
            1,
            Some(Origin::List),
            Some(2),
        ))
    );
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct TestStruct {
    field: i32,
}

#[test]
fn test_struct() {
    let data = parse_with_reader("field: 42").unwrap();
    let result = TestStruct::deserialize(data.deserializer());
    assert_eq!(result, Ok(TestStruct { field: 42 }));

    let data = parse_with_reader("key: \n\tfield: 42\n\tkey: 15").unwrap();
    let view = data.view().map().unwrap().get("key").unwrap();
    let result = TestStruct::deserialize(Deserializer::new(view));
    assert_eq!(
        result,
        Err(marked::DeserializeError::new_invalid_value(
            Mark::new(1, 1),
            r#"value of type "TestStruct""#.into(),
            Some(Box::new(marked::DeserializeError::new_unknown_key(
                Mark::new(1, 1),
                "key".into(),
                &["field"] as &[_]
            )))
        ))
    );
}

#[test]
fn test_enum() {
    let data = parse_with_reader("= Ok: 42").unwrap();
    let result = Result::<u8, u8>::deserialize(data.deserializer());
    assert_eq!(result, Ok(Ok(42)));

    let data = parse_with_reader("key: 15").unwrap();
    let view = data.view().map().unwrap().get("key").unwrap();
    let result = Result::<u8, u8>::deserialize(Deserializer::new(view));
    assert_eq!(
        result,
        Err(marked::DeserializeError::new_invalid_value(
            Mark::new(0, 5),
            r#"value of type "Result""#.into(),
            Some(Box::new(marked::DeserializeError::new_unknown_raw(
                Mark::new(0, 5),
                "15".into(),
                &["Ok", "Err"] as &[_]
            )))
        ))
    );
}
