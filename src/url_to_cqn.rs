use crate::cqn;
use crate::cqn::CQN;
use tide;

struct Parsed {
    entity: String,
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

    let root_segment: &str = match path_wo_leading_slash.find("/") {
        None => &path_wo_leading_slash,
        Some(idx) => &path_wo_leading_slash[..idx],
    };

    let parsed: Parsed = match root_segment.find("(") {
        Some(start_idx) => {
            let mut parsed = Parsed {
                entity: root_segment[..start_idx].to_string(),
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
            entity: root_segment.to_string(),
            key_vals: vec![],
        },
    };
    // println!("arguments: {:?}", arguments);
    let mut select = cqn::SELECT::from(&parsed.entity);
    for key_val in parsed.key_vals {
        select.filter(vec![&key_val.key, "=", &key_val.val]);
    }
    // println!("cqn: {:?}", cqn);
    // println!("sql: {:?}", cqn.to_sql());
    Ok(select)
}

pub fn parse(method: tide::http::Method, uri: &tide::http::Url) -> Result<CQN, UriError> {
    match method {
        tide::http::Method::Get => Ok(cqn::CQN::SELECT(get(uri)?)),
        _ => Err(UriError::NotImplemented),
    }
}
