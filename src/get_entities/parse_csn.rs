// use crate::entity;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Service {
    name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Entity {
    name: String,
    elements: Vec<Element>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Element {
    name: String,
    key: bool,
    kind: ElementKind,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "type")]
pub enum ElementKind {
    #[serde(rename = "cds.UUID")]
    UUID(PrimitiveKind<String>),
    #[serde(rename = "cds.Boolean")]
    Boolean(PrimitiveKind<bool>),
    #[serde(rename = "cds.Integer")]
    Integer(PrimitiveKind<i64>),
    #[serde(rename = "cds.String")]
    String(PrimitiveKindString),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PrimitiveKind<T> {
    default: Option<Default<T>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PrimitiveKindString {
    default: Option<Default<String>>,
    length: Option<u64>,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Default<T> {
    // TODO: other possibilities
    #[serde(rename = "val")]
    Val(T),
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "kind")]
pub enum Definition {
    Service(Service),
    Entity(Entity),
}

pub fn get_definitions(csn: &str) -> Vec<Definition> {
    let mut definitions = vec![];
    let csn_json: serde_json::value::Value = serde_json::from_str(csn).unwrap();
    let map = csn_json["definitions"].as_object().unwrap();
    for (key, val) in map {
        println!("{}", key);
        if val["kind"] == "service" {
            println!("found service");
            definitions.push(Definition::Service(Service { name: key.clone() }));
        } else if val["kind"] == "entity" {
            let mut elements: Vec<Element> = vec![];
            for (el_key, el_val) in val["elements"].as_object().unwrap() {
                println!("found el_val {}", el_val);
                let el_val_str = &el_val.to_string();
                let element_kind: ElementKind = serde_json::from_str(el_val_str).unwrap();
                let element = Element {
                    name: el_key.to_string(),
                    key: el_val["key"] == true,
                    kind: element_kind,
                };
                elements.push(element);
            }
            definitions.push(Definition::Entity(Entity {
                name: key.clone(),
                elements: elements,
            }))
        }
    }
    definitions
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_csn() -> &'static str {
        r#"{"definitions": {
            "TestService": {
              "@source": "srv/service.cds",
              "kind": "service"
            },
            "TestService.TestEntity": {
              "kind": "entity",
              "elements": {
                "ID": {
                  "key": true,
                  "type": "cds.UUID"
                },
                "name": {
                  "type": "cds.String",
                  "default": {
                    "val": "myDefaultName"
                  }
                },
                "age": {
                  "type": "cds.Integer"
                }
              }
            }
          },
          "meta": {
            "creator": "CDS Compiler v1.25.0"
          },
          "$version": "1.0"}"#
    }

    #[test]
    fn deserialize_uuid() {
        let input_str = r#"{"type": "cds.UUID", "key": true}"#;
        let deserialized: ElementKind = serde_json::from_str(input_str).unwrap();

        match deserialized {
            ElementKind::UUID(a) => assert_eq!(a.default.is_none(), true),
            _ => panic!("Could not deserialize"),
        }
    }

    #[test]
    fn deserialize_with_default() {
        let input_str = r#"{"type": "cds.UUID", "default": { "val": "defaultUUID" }}"#;
        let deserialized: ElementKind = serde_json::from_str(input_str).unwrap();

        match deserialized {
            ElementKind::UUID(a) => match a.default {
                Some(Default::Val(default)) => assert_eq!(default, "defaultUUID"),
                _ => panic!("Could not deserialize default"),
            },
            _ => panic!("Could not deserialize"),
        }
    }

    #[test]
    fn deserialize_string_with_length() {
        let input_str = r#"{"type": "cds.String", "key": true, "length": 255}"#;
        let deserialized: ElementKind = serde_json::from_str(input_str).unwrap();

        match deserialized {
            ElementKind::String(a) => match a.length {
                Some(length) => assert_eq!(length, 255),
                _ => panic!("Could not deserialize length"),
            },
            _ => panic!("Could not deserialize"),
        }
    }

    #[test]
    fn test_get_csn() {
        let csn = get_test_csn();
        let definitions = get_definitions(csn);
        println!("Found definitions");
        println!("{:?}", definitions);
        assert_eq!(1, 1);
    }
}
