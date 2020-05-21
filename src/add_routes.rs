use crate::cqn::Crunch;
use crate::cqn_to_result;
use crate::url_to_cqn;
use crate::State;
use tide::{Server, StatusCode};

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
            .all(|mut req: tide::Request<State>| async move {
                let body = req.body_json().await?;
                let state = req.state();
                let method = req.method();
                let uri = req.uri();

                match url_to_cqn::parse(method, uri, body).await {
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
