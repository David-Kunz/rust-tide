use crate::cqn;
use crate::cqn::SQL;
use serde_json::{Map, Value};
use sqlx::Error;
use sqlx_core::cursor::Cursor;
use sqlx_core::row::Row;

pub async fn cqn_to_result(cqn: &cqn::CQN, pool: &sqlx::SqlitePool) -> Result<Vec<Value>, Error> {
    match cqn {
        cqn::CQN::SELECT(select) => {
            let sql = &select.to_sql();
            let mut res = vec![];
            let mut cursor = sqlx::query(&sql).fetch(pool);
            while let Some(row) = cursor.next().await? {
                let mut map = Map::new();
                for col in &select.columns {
                    println!("col: {:?}", col);
                    let val: &str = row.get(col.as_str());
                    println!("val: {:?}", val);
                    map.insert(col.to_string(), Value::String(val.to_string()));
                }
                let obj = Value::Object(map);
                res.push(obj);
            }
            Ok(res)
        }
    }
}
