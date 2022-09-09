use serde_derive::{Serialize, Deserialize};
use std::collections::HashMap;
// Details the schema definition to manage versions of the parsing system
// The schema is used to ingest configuration information for a specific schema version allowing
// the user to set the configurable types of elements used within a given schema. 
//
// TODO: The configuration loaded needs to generate an exepectation structure that can be used to
// ingest configuration templates confirming to the specified components. For now this will be
// hardcoded to brach the gap.

///Defines the allowed structure and configuration of a specific schema version in the system. Used
///by the configuration parser to check syntax is correct
#[derive(Serialize, Deserialize, Debug)]
pub struct SchemaTemplate {
    pub version: Version,
    pub components: Vec<ComponentSchema>,
    pub resolvers: Vec<ResolverSchema>,
    pub metadata: MetadataSchema,
}

pub type Version = String;
pub type Kind = String;


// Defines the types allowed in the definition of a schema
#[derive(Debug, Serialize, Deserialize)]
pub enum AttributeType {
    Str,
    Bool,
    Array,
    Int,
    Float,
    Reference,
    Struct,
}
///Attributes define available attributes on scheme structs
#[derive(Debug, Serialize, Deserialize)]
pub struct AttributeSchema {
    name: String, 
    required: Option<bool>,
    value_type: AttributeType,
}
/// Defines supported constructs on a given schema
#[derive(Debug, Serialize, Deserialize)]
pub struct ComponentSchema {
    name: String,
    kind: Vec<Kind>,
    metadata: Option<MetadataSchema>, 
    attributes: Vec<AttributeSchema>
}

/// Allowed data to store as metadata on a configuration schema 
#[derive(Debug, Serialize, Deserialize)]
pub struct MetadataSchema {
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
