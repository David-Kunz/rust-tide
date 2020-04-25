// use crate::entity;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

fn get_test_csn_json() -> serde_json::value::Value {
    json!({
      "definitions": {
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
      "$version": "1.0"
    })
}


#[derive(Deserialize, Serialize, Debug)]
struct Service {
  name: String
}

#[derive(Deserialize, Serialize, Debug)]
struct Entity {
    name: String,
    elements: Vec<Element>,
}

#[derive(Deserialize, Serialize, Debug)]
struct PrimitiveElementKind<T> {
    #[serde(default = "make_false")]
    key: bool,
    default: Option<Default<T>>,
}

#[derive(Deserialize, Serialize, Debug)]
enum Default<T> {
    // TODO: other possibilities
    #[serde(rename = "val")]
    Val(T),
}

#[derive(Deserialize, Serialize, Debug)]
struct StringElementKind {
    #[serde(default = "make__")]
    name: String,
    #[serde(default = "make_false")]
    key: bool,
    // TODO: Better default value
    length: Option<u16>,
    default: Option<Default<String>>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Element {
  name: String,
  kind: ElementKind,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "type")]
enum ElementKind {
    #[serde(rename = "cds.UUID")]
    UUID(PrimitiveElementKind<String>),
    #[serde(rename = "cds.Boolean")]
    Boolean(PrimitiveElementKind<bool>),
    #[serde(rename = "cds.Integer")]
    Integer(PrimitiveElementKind<i64>),
    #[serde(rename = "cds.String")]
    String(StringElementKind),
}

fn make_false() -> bool {
    false
}

fn make__() -> String {
    "_".to_string()
}

fn make_1000() -> u16 {
    1000
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "kind")]
enum Definition {
    Service(Service),
    Entity(Entity),
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn serialize() {
        let element = ElementKind::UUID(PrimitiveElementKind {
            key: false,
            default: None,
        });
        let serialized = serde_json::to_string_pretty(&element).unwrap();
        println!("serialized: {}", serialized);
    }

    #[test]
    fn serialize_with_default() {
        let element = ElementKind::UUID(PrimitiveElementKind {
            key: false,
            default: Some(Default::Val("myDefaultValue".to_string())),
        });
        let serialized = serde_json::to_string_pretty(&element).unwrap();
        println!("serialized: {}", serialized);
    }

    #[test]
    fn deserialize_uuid() {
        let input_str = r#"{"type": "cds.UUID", "key": true}"#;
        let deserialized: ElementKind = serde_json::from_str(input_str).unwrap();
        if let ElementKind::UUID(el) = deserialized {
            assert_eq!(el.key, true)
        } else {
            panic!("Should have serialized to UUID")
        };
    }

    #[test]
    fn deserialize_uuid_without_key() {
        let input_str = r#"{"type": "cds.UUID"}"#;
        let deserialized: ElementKind = serde_json::from_str(input_str).unwrap();
        if let ElementKind::UUID(el) = deserialized {
            assert_eq!(el.key, false)
        } else {
            panic!("Should have serialized to UUID")
        };
    }

    #[test]
    fn deserialize_uuid_with_default() {
        let input_str = r#"{"type": "cds.UUID", "default": { "val": "defaultUUID" }}"#;
        let deserialized: ElementKind = serde_json::from_str(input_str).unwrap();
        if let ElementKind::UUID(el) = deserialized {
            assert_eq!(el.key, false);
            if let Some(Default::Val(default_val)) = el.default {
                assert_eq!(default_val, "defaultUUID");
            } else {
              panic!("Should have serialized default UUID")
            }
        } else {
            panic!("Should have serialized to UUID")
        };
    }

    #[test]
    fn deserialize_string_without_key() {
        let input_str = r#"{"type": "cds.String", "length": 255, "default": { "val": "myDefaultString" }}"#;
        let deserialized: ElementKind = serde_json::from_str(input_str).unwrap();
        if let ElementKind::String(el) = deserialized {
            assert_eq!(el.key, false);
            assert_eq!(el.length, Some(255));
            if let Some(Default::Val(default_val)) = el.default {
                assert_eq!(default_val, "myDefaultString");
            } else {
              panic!("Should have serialized default String")
            }
        } else {
            panic!("Should have serialized to String")
        };
    }

    #[test]
    fn test_get_csn() {
        let mut definitions: Vec<Definition> = vec![];
        let csn_json = get_test_csn_json();
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
              elements.push(Element {name: el_key.clone(), kind: element_kind });
            }
            definitions.push(Definition::Entity(Entity { name: key.clone(), elements: elements }))
          }
        }
        println!("Found definitions");
        println!("{:?}", definitions);
        assert_eq!(1,1);
    }
}
