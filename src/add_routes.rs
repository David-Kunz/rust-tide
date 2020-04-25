use crate::entity;
use tide::{Server, StatusCode};
mod deserialize_path;

pub fn add_routes(app: &mut Server<Vec<entity::Entity>>) -> () {
    app.at("/")
        .get(|_req: tide::Request<Vec<entity::Entity>>| async move {
            Ok(tide::Response::new(StatusCode::Ok)
                .body_string("Please use proper routes.".to_string()))
        });

    app.at("*")
        .get(|req: tide::Request<Vec<entity::Entity>>| async move {
            let uri = req.uri();

            let option_entity = deserialize_path::deserialize_path(uri, req.state());

            match option_entity {
                Err(deserialize_path::PathSerializationError::EntityNotFound(entity)) => {
                    Ok(tide::Response::new(StatusCode::NotFound)
                        .body_string(format!("The entity {} is not found", entity).to_string()))
                }
                Err(deserialize_path::PathSerializationError::KeyInvald(key)) => {
                    Ok(tide::Response::new(StatusCode::NotFound)
                        .body_string(format!("The key {} is invalid", key).to_string()))
                }
                Ok(path) => Ok(tide::Response::new(StatusCode::Ok)
                    .body_json(&path)
                    .unwrap()),
            }
        });
}
