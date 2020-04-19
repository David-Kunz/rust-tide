use async_std::io;
use async_std::task;
use std::vec::Vec;
use tide::Server;

mod deserialize_path;
mod entity;
mod get_entities;

fn main() -> io::Result<()> {
    let entities = get_entities::get_entities();
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
                        return tide::Response::new(404)
                            .body_string(format!("The entity {} is not found", entity).to_string())
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
