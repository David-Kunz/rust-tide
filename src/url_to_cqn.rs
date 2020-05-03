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

pub fn parse(method: tide::http::Method, uri: &tide::http::Url) -> Result<CQN, UriError> {
    // let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
    // let path_segments_str: Vec<&str> = uri.path().split('/').skip(1).collect();
    if method != tide::http::Method::Get {
        return Err(UriError::NotImplemented);
    }
    let path = uri.path();

    let path_wo_leading_slash: &str = &path[1..];

    let rootSegment: &str = match path_wo_leading_slash.find("/") {
        None => &path_wo_leading_slash,
        Some(idx) => {
            &path_wo_leading_slash[..idx]
        }
    };

    println!(">>>>>\n");
    println!("rootSegment: {}", rootSegment);
    let parsed: Parsed = match rootSegment.find("(") {
        Some(startIdx) => {
            let mut parsed = Parsed {
                entity: rootSegment[..startIdx].to_string(),
                key_vals: vec![],
            };
            match rootSegment.find(")") {
                Some(endIdx) => {
                    let key_vals = rootSegment[startIdx + 1..endIdx].split(",");
                    for key_val in key_vals {
                        let keyval: Vec<&str> = key_val.split("=").collect();
                        parsed.key_vals.push(KeyVal {
                            key: keyval[0].to_string(),
                            val: keyval[1].to_string(),
                        });
                    }
                    parsed
                },
                None => return Err(UriError::InvalidURI)
            }
        }
        None => Parsed {
            entity: rootSegment.to_string(),
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
    Ok(cqn::CQN::SELECT(select))
}
