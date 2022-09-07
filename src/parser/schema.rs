use serde_derive::{Serialize, Deserialize};
use std::collections::HashMap;
use super::component::Component;
use super::schemaparser::SchemaTemplate;
//Details the schema definition to manage versions of the parsing system
//

///Defines the allowed structure and configuration of a specific schema version in the system. Used
///by the configuration parser to check syntax is correct
pub struct Schema<'a, T> {
    version: Version,
    components: Option<&'a [Component<T>]>
}
impl<'a, T> Schema<'a, T> {
    pub fn new(version: String, components: Option<&'a[Component<T>]>) -> Self {
        Schema {
            version,
            components
        }
    }
    ///Loads a schema template specifying the allowed components in a given version of the schema 
    pub fn load(&mut self, template: SchemaTemplate) {
        unimplemented!();
    }
}

pub type Version = String;

// Defines the types allowed in the definition of a schema
#[derive(Debug, Serialize, Deserialize)]
enum AttributeType {
    Str,
    Bool,
    Array,
    Int,
    Float
}
///Attributes define available attributes on scheme structs
#[derive(Debug, Serialize, Deserialize)]
struct AttributeSchema {
    name: String, 
    value_type: AttributeType,
}
/// Defines supported constructs on a given schema
#[derive(Debug, Serialize, Deserialize)]
struct ComponentSchema {
    name: String,
    attributes: Vec<AttributeSchema>
}

/// Allowed data to store as metadata on a configuration schema 
#[derive(Debug, Serialize, Deserialize)]
struct MetadataSchema {
    required: bool,
    attributes: HashMap<String, AttributeSchema>
}
/// A supported resolver type to utilize to resolve values in the schema
#[derive(Debug, Serialize, Deserialize)]
struct ResolverSchema {
    name: String,
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
