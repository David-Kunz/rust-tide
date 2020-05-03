use tide::{Server, StatusCode};
use crate::url_to_cqn;
use crate::State;
use serde::Serialize;
use sqlx::sqlite::SqliteQueryAs;
use crate::cqn;
use crate::cqn::SQL;

#[derive(sqlx::FromRow, Debug, Serialize)]
struct MyEntity {
    ID: String,
    name: String,
    age: i64,
}

// #[derive(sqlx::FromRow)]
// struct Todo { id: i64, desscription: String, done: bool }

pub fn add_routes(app: &mut Server<State>) -> () {
    app.at("/").get(|_req: tide::Request<State>| async move {
        Ok(
            tide::Response::new(StatusCode::Ok)
                .body_string("Please use proper routes.".to_string()),
        )
    });

    app.at("*").get(|req: tide::Request<State>| async move {
        let uri = req.uri();
        let method = req.method();

        let state = req.state();

        match url_to_cqn::parse(method, uri) {
            Ok(cqn) => {
                match cqn {
                    cqn::CQN::SELECT(select) => {
                        println!("found cqn {:?}", &select.to_sql());
                        let res = sqlx::query_as::<_, MyEntity>(&select.to_sql())
                            .fetch_all(&state.pool)
                            .await?;
                        println!("found: {:?}", res);
                        return Ok(tide::Response::new(StatusCode::Ok).body_json(&res).unwrap());
                    }
                }
            }
            Err(url_to_cqn::UriError::InvalidURI) => {
                Ok(tide::Response::new(StatusCode::BadRequest)
                    .body_string("Bad request".to_string()))
            },
            Err(url_to_cqn::UriError::NotImplemented) => {
                Ok(tide::Response::new(StatusCode::NotImplemented)
                    .body_string("Bad request".to_string()))
            }
        }

        // let option_entity = deserialize_path::deserialize_path(uri, &state.entities);

        // match option_entity {
        //     Err(deserialize_path::PathSerializationError::EntityNotFound(entity)) => {
        //         Ok(tide::Response::new(StatusCode::NotFound)
        //             .body_string(format!("The entity {} is not found", entity).to_string()))
        //     }
        //     Err(deserialize_path::PathSerializationError::KeyInvald(key)) => {
        //         Ok(tide::Response::new(StatusCode::NotFound)
        //             .body_string(format!("The key {} is invalid", key).to_string()))
        //     }
        //     Ok(path) => {
        //         println!("OK Path");

        //         let deserialize_path::Path(pe) = &path;
        //         match pe {
        //             deserialize_path::PathElement::EntityWithKeys(entity_w_keys) => {
        //             //   sqlx::query("SELECT FROM ?").bind(entity.).execute(&mut conn).await?;
        //                 println!("nyi");
        //             },
        //             deserialize_path::PathElement::EntityWithoutKeys(entity_wo_keys) => {
        //                 println!("OK EntityWOKeys");
        //                 let entity = entity_wo_keys.entity;
        //                 // let insert = sqlx::query("INSERT INTO todos (description) VALUES ( $1 )").bind(&entity.name).execute(&state.pool).await;
        //                 // println!("found: {:?}", insert);
        //                 // let get = sqlx::query_as::<_, MyEntity>("SELECT * FROM MyService_MyEntity");
        //                 let res = sqlx::query_as::<_, MyEntity>("SELECT * FROM MyEntity")
        //                 .fetch_all(&state.pool).await?;
        //                 // .execute(&state.pool).await;
        //                 println!("found: {:?}", res);
        //                 return Ok(tide::Response::new(StatusCode::Ok)
        //                 .body_json(&res)
        //                 .unwrap())
        //             }
        //         }
        //         // let deserialize_path::PathElement(a)= path;
        //         // match deserialize_path::Path(path) {
        //         //     deserialize_path::PathElement(pa) => {},
        //         // }
        //         Ok(tide::Response::new(StatusCode::Ok)
        //         .body_json(&path)
        //         .unwrap())
        //     }
        // }
    });
}
