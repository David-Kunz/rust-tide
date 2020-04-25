use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug)]
pub struct Entity {
    pub name: String,
    pub elements: Vec<Element>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Element {
    pub name: String,
    pub el_type: ElementType,
    pub is_key: bool,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "kind")]
pub enum Definition {
    Service(Service),
    Entity(Entity),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Service {
    Kind: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum ElementType {
    UUID,
    Boolean,
    Integer,
    Integer64,
    Decimal,
    DecimalFloat,
    Double,
    Date,
    Time,
    DateTime,
    Timestamp,
    String,
    Binary,
    LargeBinary,
    LargeString,
}
