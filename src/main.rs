use async_std::io;
use async_std::task;
use serde::{Deserialize, Serialize};
use std::vec::Vec;
use tide::Server;

#[derive(Deserialize, Serialize)]
struct Entity {
    name: String,
    elements: Vec<Element>,
}

#[derive(Deserialize, Serialize)]
struct Element {
    name: String,
    el_type: ElementType,
    is_key: bool,
}

#[derive(Deserialize, Serialize)]
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

fn determine_entity<'a>(uri: &tide::http::Uri, entities: &'a Vec<Entity>) -> Option<&'a Entity> {
    let mut entity_c = vec![];
    for c in uri.path().chars() {
        match c {
            '(' => {},
            ')' => {},
            '/' => {entity_c = vec![]},
            _ => entity_c.push(c)
        }
    }
    let entity: String = entity_c.iter().collect();
    entities.iter().find(|e| e.name == entity)
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
                if option_entity.is_none() {
                    return tide::Response::new(404).body_string("404, not found".to_string());
                }
                tide::Response::new(200)
                    .body_json(option_entity.unwrap())
                    .unwrap()
            });

        let url = "127.0.0.1:8080";
        println!("Server listening on http://{}", &url);
        app.listen(&url).await?;
        Ok(())
    })
}