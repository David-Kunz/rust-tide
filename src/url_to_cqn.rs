use crate::cqn;
use crate::cqn::CQN;
use tide;

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

fn get(uri: &tide::http::Url) -> Result<cqn::SELECT, UriError> {
    let path = uri.path();

    let path_wo_leading_slash: &str = &path[1..];

    let idx_of_root_segment_res = path_wo_leading_slash.find("/");

    match idx_of_root_segment_res {
        None => {
            return Err(UriError::InvalidURI);
        }
        Some(idx_of_snd_slash) => {
            let path_wo_service_res = match path_wo_leading_slash.find("/") {
                None => Err(UriError::InvalidURI),
                Some(idx) => Ok(&path_wo_leading_slash[idx + 1..]),
            };

            match path_wo_service_res {
                Err(e) => Err(e),
                Ok(path_wo_service) => {
                    let root_segment: &str = match path_wo_service.find("/") {
                        None => &path_wo_service,
                        Some(idx) => &path_wo_service[..idx],
                    };

                    println!("root segment: {}", root_segment);
                    let parsed: Parsed = match root_segment.find("(") {
                        Some(start_idx) => {
                            let mut parsed = Parsed {
                                name: root_segment[..start_idx].to_string(),
                                key_vals: vec![],
                            };
                            match root_segment.find(")") {
                                Some(end_idx) => {
                                    let key_vals = root_segment[start_idx + 1..end_idx].split(",");
                                    for key_val in key_vals {
                                        let keyval: Vec<&str> = key_val.split("=").collect();
                                        parsed.key_vals.push(KeyVal {
                                            key: keyval[0].to_string(),
                                            val: keyval[1].to_string(),
                                        });
                                    }
                                    parsed
                                }
                                None => return Err(UriError::InvalidURI),
                            }
                        }
                        None => Parsed {
                            name: root_segment.to_string(),
                            key_vals: vec![],
                        },
                    };
                    let service_name = &path_wo_leading_slash[..idx_of_snd_slash];
                    let entity_name = format!("{}_{}", service_name, parsed.name);
                    let mut select = cqn::SELECT::from(&entity_name);
                    for key_val in parsed.key_vals {
                        select.filter(vec![&key_val.key, "=", &key_val.val]);
                    }
                    Ok(select)
                }
            }
        }
    }
}

pub fn parse(method: tide::http::Method, uri: &tide::http::Url) -> Result<CQN, UriError> {
    match method {
        tide::http::Method::Get => Ok(cqn::CQN::SELECT(get(uri)?)),
        _ => Err(UriError::NotImplemented),
    }
}
