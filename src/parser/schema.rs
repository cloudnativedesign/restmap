use serde_derive::{Serialize, Deserialize};
use std::collections::HashMap;
//Details the schema definition to manage versions of the parsing system
//

///Defines the allowed structure and configuration of a specific schema version in the system. Used
///by the configuration parser to check syntax is correct
#[derive(Serialize, Deserialize, Debug)]
pub struct SchemaTemplate {
    version: Version,
    metadata: MetadataSchema, 
    components: Vec<ComponentSchema>,
    resolvers: Vec<ResolverSchema>,
}

pub type Version = String;


// Defines the types allowed in the definition of a schema
#[derive(Debug, Serialize, Deserialize)]
pub enum AttributeType {
    Str,
    Bool,
    Array,
    Int,
    Float
}
///Attributes define available attributes on scheme structs
#[derive(Debug, Serialize, Deserialize)]
pub struct AttributeSchema {
    name: String, 
    value_type: AttributeType,
}
/// Defines supported constructs on a given schema
#[derive(Debug, Serialize, Deserialize)]
pub struct ComponentSchema {
    name: String,
    attributes: Vec<AttributeSchema>
}

/// Allowed data to store as metadata on a configuration schema 
#[derive(Debug, Serialize, Deserialize)]
pub struct MetadataSchema {
    required: bool,
    attributes: HashMap<String, AttributeSchema>
}
/// A supported resolver type to utilize to resolve values in the schema
#[derive(Debug, Serialize, Deserialize)]
pub struct ResolverSchema {
    name: String,
    attributes: Vec<AttributeSchema>
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instantiate_schema() {
        unimplemented!()
    }

    #[test]
    fn instantiate_version() {
        let version_010: Version = "0.1.0".to_string();
    }
}
