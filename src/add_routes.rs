use crate::cqn::Crunch;
use crate::cqn_to_result;
use crate::url_to_cqn;
use crate::State;
use serde::Serialize;
use tide::{Server, StatusCode};

// TODO: Should be generated from CSN
#[derive(sqlx::FromRow, Debug, Serialize)]
struct MyEntity {
    ID: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    age: Option<i64>,
}

pub fn add_routes(app: &mut Server<State>, service_names: Vec<String>) -> () {
    app.at("/").get(|_req: tide::Request<State>| async move {
        Ok(
            tide::Response::new(StatusCode::Ok)
                .body_string("Please use proper routes.".to_string()),
        )
    });

    for service_name in service_names {
        let endpoint = format!("{}{}", service_name, "/*");
        app.at(&endpoint)
            .get(|req: tide::Request<State>| async move {
                let uri = req.uri();
                let method = req.method();
                let state = req.state();

                match url_to_cqn::parse(method, uri) {
                    Ok(mut cqn) => {
                        cqn.crunch(&state.definitions);
                        let res = cqn_to_result::cqn_to_result(&cqn, &state.pool).await?;
                        return Ok(tide::Response::new(StatusCode::Ok).body_json(&res).unwrap());
                    }
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
}
