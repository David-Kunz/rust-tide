use async_std::io;
use async_std::task;
use serde::{Deserialize, Serialize};
use std::vec::Vec;
use tide::Server;

#[derive(Deserialize, Serialize, Debug)]
struct Entity {
    name: String,
    elements: Vec<Element>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Element {
    name: String,
    el_type: ElementType,
    is_key: bool,
}

#[derive(Deserialize, Serialize, Debug)]
enum ElementType {
    UUID,
    Boolean,
    Integer,
    Integer64,
    Decimal,
    DecimalFloat,
    Double,
    Date,
    Time,
    DateTime,
    Timestamp,
    String,
    Binary,
    LargeBinary,
    LargeString,
}

#[derive(Serialize)]
struct Path<'a>(PathElement<'a>);

#[derive(Serialize)]
enum PathElement<'a> {
    EntityWithoutKeys(EntityWithoutKeys<'a>),
    EntityWithKeys(EntityWithKeys<'a>),
}

#[derive(Serialize)]
struct EntityWithoutKeys<'a> {
    entity: &'a Entity,
}

#[derive(Serialize)]
struct EntityWithKeys<'a> {
    entity: &'a Entity,
    keys: Vec<KeyVal<'a>>,
    // nav: Option<Navigation>,
}

// struct Navigation {
//     name: String,
//     nav: Option<Box<Navigation>>,
// }

#[derive(Serialize)]
struct KeyVal<'a> {
    key: &'a Element,
    val: String,
}

enum DerivingError {
    KeyInvald(String),
    EntityNotFound(String),
    InvalidPath,
}

fn determine_entity<'a>(
    uri: &tide::http::Uri,
    entities: &'a Vec<Entity>,
) -> Result<Path<'a>, DerivingError> {
    // /entity1(key1=1,key2=4)/entity2
    let path_segments_str: Vec<&str> = uri.path().split('/').skip(1).collect();
    println!("path_segments_str {:?}", &path_segments_str);

    // [entity1(key1=1,key2=4), entity2]
    for path_segment in path_segments_str {
        // [entity1, key1=1,key2=4]
        let brackets_strs: Vec<&str> = path_segment.split(|c| c == '(' || c == ')').collect();
        let entity_str: &str = brackets_strs[0];
        let entity_csn = entities.iter().find(|e| e.name == entity_str);

        match entity_csn {
            Some(found_entity_csn) => {
                if brackets_strs.len() > 1 {
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
                            },
                            None => {
                                return Err(DerivingError::KeyInvald(key_str.to_string()))
                            }
                        }
                    }
                    let entity = EntityWithKeys {
                        entity: found_entity_csn,
                        keys: keys,
                    };
                    let path = Path(PathElement::EntityWithKeys(entity));
                    return Ok(path);
                } else {
                    let path = Path(PathElement::EntityWithoutKeys(EntityWithoutKeys {
                        entity: found_entity_csn,
                    }));
                    return Ok(path);
                }
            }
            None => {
                return Err(DerivingError::EntityNotFound(entity_str.to_string()));
            }
        }
    }
    return Err(DerivingError::InvalidPath);
}

fn main() -> io::Result<()> {
    // TODO: Load from external csn file
    let entities = vec![
        Entity {
            name: "entity1".to_string(),
            elements: vec![
                Element {
                    name: "sub11".to_string(),
                    el_type: ElementType::UUID,
                    is_key: true,
                },
                Element {
                    name: "sub12".to_string(),
                    is_key: false,
                    el_type: ElementType::String,
                },
            ],
        },
        Entity {
            name: "entity2".to_string(),
            elements: vec![
                Element {
                    name: "sub21".to_string(),
                    is_key: true,
                    el_type: ElementType::UUID,
                },
                Element {
                    name: "sub22".to_string(),
                    is_key: false,
                    el_type: ElementType::Integer,
                },
            ],
        },
        Entity {
            name: "entity3".to_string(),
            elements: vec![
                Element {
                    name: "sub31".to_string(),
                    is_key: true,
                    el_type: ElementType::UUID,
                },
                Element {
                    name: "sub32".to_string(),
                    is_key: false,
                    el_type: ElementType::Integer64,
                },
            ],
        },
    ];
    task::block_on(async {
        let mut app = Server::with_state(entities);

        app.at("/")
            .get(|_req: tide::Request<Vec<Entity>>| async move {
                tide::Response::new(200).body_string("Please use proper routes.".to_string())
            });

        app.at("*")
            .get(|req: tide::Request<Vec<Entity>>| async move {
                let uri = req.uri();

                let option_entity = determine_entity(uri, req.state());

                match option_entity {
                    Err(DerivingError::InvalidPath) => {
                        return tide::Response::new(404)
                            .body_string("Your path is invalid".to_string());
                    }
                    Err(DerivingError::EntityNotFound(entity)) => {
                        return tide::Response::new(404).body_string(
                            format!("The entity {} is not found", entity).to_string(),
                        );
                    }
                    Err(DerivingError::KeyInvald(key)) => {
                        return tide::Response::new(404)
                            .body_string(format!("The key {} is invalid", key).to_string());
                    }
                    Ok(path) => tide::Response::new(200).body_json(&path).unwrap(),
                }
            });

        let url = "127.0.0.1:8080";
        println!("Server listening on http://{}", &url);
        app.listen(&url).await?;
        Ok(())
    })
}
