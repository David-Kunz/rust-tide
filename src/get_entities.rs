use crate::entity;
use crate::parse_csn;

pub fn get_entities() -> Vec<entity::Entity> {
    let entities = vec![
        entity::Entity {
            name: "entity1".to_string(),
            elements: vec![
                entity::Element {
                    name: "sub11".to_string(),
                    el_type: entity::ElementType::UUID,
                    is_key: true,
                },
                entity::Element {
                    name: "sub12".to_string(),
                    is_key: false,
                    el_type: entity::ElementType::String,
                },
            ],
        },
        entity::Entity {
            name: "entity2".to_string(),
            elements: vec![
                entity::Element {
                    name: "sub21".to_string(),
                    is_key: true,
                    el_type: entity::ElementType::UUID,
                },
                entity::Element {
                    name: "sub22".to_string(),
                    is_key: false,
                    el_type: entity::ElementType::Integer,
                },
            ],
        },
        entity::Entity {
            name: "entity3".to_string(),
            elements: vec![
                entity::Element {
                    name: "sub31".to_string(),
                    is_key: true,
                    el_type: entity::ElementType::UUID,
                },
                entity::Element {
                    name: "sub32".to_string(),
                    is_key: false,
                    el_type: entity::ElementType::Integer64,
                },
            ],
        },
    ];
    entities
}
