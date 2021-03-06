use crate::csn;
use serde_json::Value;

#[derive(Debug)]
pub enum CQN {
    SELECT(SELECT),
    INSERT(INSERT),
}
#[derive(Debug)]
pub struct SELECT {
    pub entity: Identifier,
    pub columns: Vec<Identifier>,
    pub filter: Vec<String>,
}

#[derive(Debug)]
pub struct INSERT {
    pub entity: String,
    pub data: Value,
    pub filter: Vec<String>,
}

#[derive(Debug)]
pub struct Identifier {
    pub reference: Vec<String>,
    pub alias: Option<String>
}

impl SELECT {
    pub fn from(entity: &str) -> SELECT {
        SELECT {
            entity: Identifier { reference: vec![entity.to_string()], alias: None },
            columns: vec![],
            filter: vec![],
        }
    }
    pub fn columns(&mut self, columns: Vec<&str>) -> &mut Self {
        let cols: Vec<Identifier> = columns
            .iter()
            .map(|col| Identifier {
                reference: vec![col.to_string()],
                alias: None
            })
            .collect();
        self.columns.extend(cols);
        self
    }

    pub fn filter(&mut self, filter: Vec<&str>) -> &mut Self {
        let filter: Vec<String> = filter.iter().map(|col| col.to_string()).collect();
        if !self.filter.is_empty() && self.filter.last() != Some(&"and".to_string()) {
            self.filter.insert(0, "(".to_string());
            self.filter.push(")".to_string());
            self.filter.push("and".to_string());
            self.filter.extend(filter);
        } else {
            self.filter.extend(filter);
        }
        self
    }
}

pub trait Crunch {
    fn crunch(&mut self, definitions: &csn::Definitions) -> &mut Self;
}

impl Crunch for CQN {
    fn crunch(&mut self, definitions: &csn::Definitions) -> &mut Self {
        match self {
            CQN::SELECT(select) => {
                let definition = definitions.definitions.iter().find(|&d| match d {
                    csn::Definition::Entity(entity) => entity.name == *select.entity.reference.last().unwrap(),
                    _ => false,
                });

                if let Some(csn::Definition::Entity(entity)) = definition {
                    for column in select.columns.iter() {
                        if let None = entity
                            .elements
                            .iter()
                            .find(|&e| &e.name == column.reference.last().unwrap())
                        {
                        }
                    }

                    // Workaround: Always set all columns if none are given
                    if select.columns.len() == 0 {
                        let all_cols = entity.elements.iter().map(|e| e.name.as_str()).collect();
                        select.columns(all_cols);
                    }
                }
            }
            CQN::INSERT(insert) => {
                let definition = definitions.definitions.iter().find(|&d| match d {
                    csn::Definition::Entity(entity) => entity.name == insert.entity,
                    _ => false,
                });
                if let Some(csn::Definition::Entity(entity)) = definition {
                    match &entity.query {
                        Some(query) => {
                            insert.entity = query.from.to_string();
                        }
                        None => {}
                    }
                }
            }
        }
        self
    }
}

pub trait SQL {
    fn to_sql(&self) -> String;
}

impl SQL for SELECT {
    fn to_sql(&self) -> String {
        let from_sql = &self.entity.reference.join(".").to_string().replace(".", "_");
        let mut res = match &self.columns.len() > &0 {
            true => {
                let cols: Vec<String> = self
                    .columns
                    .iter()
                    .map(|c| c.reference.join(".").to_string())
                    .collect();
                format!("SELECT {} FROM {}", cols.join(","), &from_sql)
            }
            false => format!("SELECT * FROM {}", &from_sql),
        };
        if &self.filter.len() > &0 {
            res = format!("{}{}", res, format!(" WHERE {}", &self.filter.join(" ")));
        }
        res
    }
}

impl SQL for INSERT {
    fn to_sql(&self) -> String {
        let into_sql = &self.entity.to_string().replace(".", "_");
        let mut values_sql: String = "(".to_string();
        let mut cols_sql: String = "(".to_string();
        for key_val in self.data.as_object().unwrap() {
            cols_sql = format!("{}{},", cols_sql, key_val.0);
            values_sql = format!("{}'{}',", values_sql, key_val.1);
        }
        cols_sql.pop();
        cols_sql = format!("{}{}", cols_sql, ")");
        values_sql.pop();
        values_sql = format!("{}{}", values_sql, ")");
        values_sql = values_sql.replace("\"", "");
        // let values_sql = &self.data.to_string();
        let res = format!(
            "INSERT INTO {} {} VALUES {}",
            &into_sql, &cols_sql, &values_sql
        );
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn select_with_col_to_sql() {
        let mut select = SELECT::from("example_entity");
        select.columns(vec!["col1", "col2"]);
        assert_eq!(select.to_sql(), "SELECT col1,col2 FROM example_entity")
    }

    #[test]
    fn select_without_col_to_sql() {
        let select = SELECT::from("example_entity");
        assert_eq!(select.to_sql(), "SELECT * FROM example_entity")
    }

    #[test]
    fn select_with_filter_to_sql() {
        let mut select = SELECT::from("example_entity");
        select.filter(vec![
            "(", "a", ">", "2", "and", "b", "<", "9", ")", "or", "c", "<", "4",
        ]);
        select.filter(vec!["d", "=", "9"]);
        assert_eq!(
            select.to_sql(),
            "SELECT * FROM example_entity WHERE ( ( a > 2 and b < 9 ) or c < 4 ) and d = 9"
        )
    }
}
