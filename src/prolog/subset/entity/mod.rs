pub mod entity_declaration;
pub mod entity_definition;
pub mod entity_value;

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub enum EntitySource {
    Internal,
    External,
    None,
}
