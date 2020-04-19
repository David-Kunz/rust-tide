use async_std::io;
use async_std::task;
use std::vec::Vec;
use tide::Server;

mod entity;
mod deserialize_path;
// mod deserialize_path;



fn main() -> io::Result<()> {
    // TODO: Load from external csn file
    let entities = vec![
        entity::Entity {
            name: "entity1".to_string(),
            elements: vec![
                entity::Element {
                    name: "sub11".to_string(),
                    el_type: entity::ElementType::UUID,
                    is_key: true,
                },
                entity::Element {
                    name: "sub12".to_string(),
                    is_key: false,
                    el_type: entity::ElementType::String,
                },
            ],
        },
        entity::Entity {
            name: "entity2".to_string(),
            elements: vec![
                entity::Element {
                    name: "sub21".to_string(),
                    is_key: true,
                    el_type: entity::ElementType::UUID,
                },
                entity::Element {
                    name: "sub22".to_string(),
                    is_key: false,
                    el_type: entity::ElementType::Integer,
                },
            ],
        },
        entity::Entity {
            name: "entity3".to_string(),
            elements: vec![
                entity::Element {
                    name: "sub31".to_string(),
                    is_key: true,
                    el_type: entity::ElementType::UUID,
                },
                entity::Element {
                    name: "sub32".to_string(),
                    is_key: false,
                    el_type: entity::ElementType::Integer64,
                },
            ],
        },
    ];
    task::block_on(async {
        let mut app = Server::with_state(entities);

        app.at("/")
            .get(|_req: tide::Request<Vec<entity::Entity>>| async move {
                tide::Response::new(200).body_string("Please use proper routes.".to_string())
            });

        app.at("*")
            .get(|req: tide::Request<Vec<entity::Entity>>| async move {
                let uri = req.uri();

                let option_entity = deserialize_path::deserialize_path(uri, req.state());

                match option_entity {
                    Err(deserialize_path::PathSerializationError::EntityNotFound(entity)) => {
                        return tide::Response::new(404).body_string(
                            format!("The entity {} is not found", entity).to_string(),
                        )
                    }
                    Err(deserialize_path::PathSerializationError::KeyInvald(key)) => {
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
