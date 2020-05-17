use crate::csn;

#[derive(Debug)]
pub enum CQN {
    SELECT(SELECT),
}
#[derive(Debug)]
pub struct SELECT {
    pub from: String,
    pub columns: Vec<String>,
    pub filter: Vec<String>,
}

impl SELECT {
    pub fn from(entity: &str) -> SELECT {
        SELECT {
            from: entity.to_string(),
            columns: vec![],
            filter: vec![],
        }
    }
    pub fn columns(&mut self, columns: Vec<&str>) -> &mut Self {
        let cols: Vec<String> = columns.iter().map(|col| col.to_string()).collect();
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
                    csn::Definition::Entity(entity) => entity.name == select.from,
                    _ => false,
                });

                if let Some(csn::Definition::Entity(entity)) = definition {
                    for column in select.columns.iter() {
                        println!("checking column {}...", column);
                        if let None = entity.elements.iter().find(|&e| &e.name == column) {
                            println!("Did not find column {}", column);
                        }
                    }

                    // Workaround: Always set all columns if none are given
                    if select.columns.len() == 0 {
                        let all_cols = entity.elements.iter().map(|e| e.name.as_str()).collect();
                        select.columns(all_cols);
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
        let from_sql = &self.from.to_string().replace(".", "_");
        let mut res = match &self.columns.len() > &0 {
            true => format!("SELECT {} FROM {}", &self.columns.join(","), &from_sql),
            false => format!("SELECT * FROM {}", &from_sql),
        };
        if &self.filter.len() > &0 {
            res = format!("{}{}", res, format!(" WHERE {}", &self.filter.join(" ")));
        }
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
