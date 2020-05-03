use crate::cqn;
use crate::cqn::SQL;
use crate::url_to_cqn;
use crate::State;
use serde::Serialize;
use sqlx::sqlite::SqliteQueryAs;
use tide::{Server, StatusCode};

#[derive(sqlx::FromRow, Debug, Serialize)]
struct MyEntity {
    ID: String,
    name: String,
    age: i64,
}

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
            Ok(cqn) => match cqn {
                cqn::CQN::SELECT(select) => {
                    println!("found cqn {:?}", &select.to_sql());
                    let res = sqlx::query_as::<_, MyEntity>(&select.to_sql())
                        .fetch_all(&state.pool)
                        .await?;
                    println!("found: {:?}", res);
                    return Ok(tide::Response::new(StatusCode::Ok).body_json(&res).unwrap());
                }
            },
            Err(url_to_cqn::UriError::InvalidURI) => {
                Ok(tide::Response::new(StatusCode::BadRequest)
                    .body_string("Bad request".to_string()))
            }
            Err(url_to_cqn::UriError::NotImplemented) => {
                Ok(tide::Response::new(StatusCode::NotImplemented)
                    .body_string("Bad request".to_string()))
            }
        }
    });
}
