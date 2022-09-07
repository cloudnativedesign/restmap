use serde_derive::Serialize;
use std::collections::HashMap;
use super::component::Component;
use super::schemaparser::SchemaTemplate;
//Details the schema definition to manage versions of the parsing system
//

///Defines the allowed structure and configuration of a specific schema version in the system. Used
///by the configuration parser to check syntax is correct
pub struct Schema<'a, T> {
    version: Version,
    components: Option<&'a [Component<'a, T>]>
}
impl<'a, T> Schema<'a, T> {
    pub fn new(version: &str, components: Option<&'a[Component<'a, T>]>) -> Self {
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
enum AttributeType {
    Str,
    Bool,
    Array,
    Int,
    Float
}
///Attributes define available attributes on scheme structs
#[derive(Serialize)]
struct AttributeSchema {
    name: &str, 
    value_type: AttributeType,
}
/// Defines supported constructs on a given schema
#[derive(Serialize)]
struct ComponentSchema<'a> {
    name: &str,
    attributes: &[AttributeSchema]
}

/// Allowed data to store as metadata on a configuration schema 
#[derive(Serialize)]
struct MetadataSchema {
    required: bool,
    attributes: HashMap<&str, AttributeSchema>
}
/// A supported resolver type to utilize to resolve values in the schema
#[derive(Serialize)]
struct ResolverSchema {
    name: &str,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instantiate_schema() {
        let schema = Schema::new("0.1.0", None);
    }

    #[test]
    fn instantiate_version() {
        let version_010: Version = "0.1.0".to_string();
    }
}
