use crate::entity;
use serde::Serialize;

// use entity;

#[derive(Serialize)]
pub struct Path<'a>(PathElement<'a>);

#[derive(Serialize)]
enum PathElement<'a> {
    EntityWithoutKeys(EntityWithoutKeys<'a>),
    EntityWithKeys(EntityWithKeys<'a>),
}

#[derive(Serialize)]
struct EntityWithoutKeys<'a> {
    entity: &'a entity::Entity,
}

#[derive(Serialize)]
struct EntityWithKeys<'a> {
    entity: &'a entity::Entity,
    keys: Vec<KeyVal<'a>>,
    // nav: Option<Navigation>,
}

// struct Navigation {
//     name: String,
//     nav: Option<Box<Navigation>>,
// }

#[derive(Serialize)]
struct KeyVal<'a> {
    key: &'a entity::Element,
    val: String,
}
pub enum PathSerializationError {
    KeyInvald(String),
    EntityNotFound(String),
}

fn get_entity_with_keys<'a>(
    brackets_strs: Vec<&'a str>,
    found_entity_csn: &'a entity::Entity,
) -> Result<Path<'a>, PathSerializationError> {
    let mut keys: Vec<KeyVal> = vec![];
    let key_val_strs: Vec<&str> = brackets_strs[1].split(',').collect();
    for key_val_str in key_val_strs {
        let key_val_split: Vec<&str> = key_val_str.split('=').collect();
        let key_str = key_val_split[0];

        let option_found_key = found_entity_csn
            .elements
            .iter()
            .find(|e| e.is_key && e.name == key_str);
        match option_found_key {
            Some(found_key) => {
                let key_val = KeyVal {
                    key: found_key,
                    val: key_val_split[1].to_string(),
                };
                keys.push(key_val);
            }
            None => return Err(PathSerializationError::KeyInvald(key_str.to_string())),
        }
    }
    let entity = EntityWithKeys {
        entity: found_entity_csn,
        keys: keys,
    };
    let path = Path(PathElement::EntityWithKeys(entity));
    return Ok(path);
}

fn get_entity_without_keys<'a>(
    found_entity_csn: &'a entity::Entity,
) -> Result<Path<'a>, PathSerializationError> {
    let path = Path(PathElement::EntityWithoutKeys(EntityWithoutKeys {
        entity: found_entity_csn,
    }));
    return Ok(path);
}

fn get_path<'a>(
    path_segment: &'a str,
    entities: &'a Vec<entity::Entity>,
) -> Result<Path<'a>, PathSerializationError> {
    // [entity1, key1=1,key2=4]
    let brackets_strs: Vec<&str> = path_segment.split(|c| c == '(' || c == ')').collect();
    let entity_str: &str = brackets_strs[0];
    let entity_csn = entities.iter().find(|e| e.name == entity_str);

    match entity_csn {
        Some(found_entity_csn) => {
            if brackets_strs.len() > 1 {
                return get_entity_with_keys(brackets_strs, found_entity_csn);
            } else {
                return get_entity_without_keys(found_entity_csn);
            }
        }
        None => {
            return Err(PathSerializationError::EntityNotFound(
                entity_str.to_string(),
            ));
        }
    }
}

pub fn deserialize_path<'a>(
    uri: &'a tide::http::Uri,
    entities: &'a Vec<entity::Entity>,
) -> Result<Path<'a>, PathSerializationError> {
    // /entity1(key1=1,key2=4)/entity2
    let path_segments_str: Vec<&str> = uri.path().split('/').skip(1).collect();
    // [entity1(key1=1,key2=4), entity2]
    get_path(path_segments_str[0], entities)
}
