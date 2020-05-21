use crate::cqn;
use crate::cqn::CQN;
use tide;
use tide::http::{Method, Url};
use serde_json::Value;

struct Parsed {
    name: String,
    key_vals: Vec<KeyVal>,
}
#[derive(Debug)]
struct KeyVal {
    key: String,
    val: String,
}

pub enum UriError {
    InvalidURI,
    NotImplemented,
}

fn parse_path(uri: &tide::http::Url) -> Result<Parsed, UriError> {
    let path = uri.path();
    let query = uri.query();

    let path_segments: Vec<&str> = path.split('/').collect();

    let service_name = path_segments[1];
    let entity_segment = path_segments[2];

    return match entity_segment.find("(") {
        Some(start_idx) => {
            let mut parsed = Parsed {
                name: entity_segment[..start_idx].to_string(),
                key_vals: vec![],
            };
            match entity_segment.find(")") {
                Some(end_idx) => {
                    let key_vals = entity_segment[start_idx + 1..end_idx].split(",");
                    for key_val in key_vals {
                        let keyval: Vec<&str> = key_val.split("=").collect();
                        parsed.key_vals.push(KeyVal {
                            key: keyval[0].to_string(),
                            val: keyval[1].to_string(),
                        });
                    }
                    Ok(parsed)
                }
                None => return Err(UriError::InvalidURI),
            }
        }
        None => Ok(Parsed {
            name: entity_segment.to_string(),
            key_vals: vec![],
        }),
    };
}

fn post(uri: &tide::http::Url, body: &Value) -> Result<cqn::INSERT, UriError> {
    let parsed = parse_path(uri)?;
    Ok(cqn::INSERT {
        into: parsed.name,
        data: body.clone(),
        filter: vec![],
    })
}

fn get(uri: &tide::http::Url) -> Result<cqn::SELECT, UriError> {
    let path = uri.path();
    let query = uri.query();

    let path_segments: Vec<&str> = path.split('/').collect();

    let service_name = path_segments[1];

    let parsed= parse_path(uri)?;
    let entity_name = format!("{}.{}", service_name, parsed.name);
    let mut select = cqn::SELECT::from(&entity_name);
    for key_val in parsed.key_vals {
        select.filter(vec![&key_val.key, "=", &key_val.val]);
    }

    let query_segs: Vec<&str> = match query {
        Some(query_seq) => query_seq.split("&").collect(),
        None => vec![],
    };

    for query_seg in query_segs {
        let param_val: Vec<&str> = query_seg.split("=").collect();
        let param = param_val.first();
        let val = param_val.last();

        match param {
            Some(&"$select") => match val {
                Some(vals) => {
                    select.columns(vals.split(",").collect());
                }
                _ => {}
            },
            Some(&"$filter") => match val {
                Some(vals) => {
                    let cleaned_vals = vals
                        .replace("eq", "=")
                        .replace("%20gt%20", "%20>%20")
                        .replace("%20lt%20", "%20<%20")
                        .replace("%20gte%20", "%20>=%20")
                        .replace("%20lte%20", "%20<=%20");
                    let vec_vals = cleaned_vals.split("%20").collect();
                    select.filter(vec_vals);
                }
                _ => {}
            },
            _ => {}
        }
    }
    Ok(select)
}

pub fn parse(method: Method, uri: &Url, body: Option<Value>) -> Result<CQN, UriError> {
    match method {
        tide::http::Method::Get => Ok(cqn::CQN::SELECT(get(uri)?)),
        tide::http::Method::Post => Ok(cqn::CQN::INSERT(post(uri, &body.unwrap())?)),
        _ => Err(UriError::NotImplemented),
    }
}
