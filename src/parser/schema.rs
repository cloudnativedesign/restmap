use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::collections::HashMap;
use component::{Component};
use attribute::*;
use configparser::*;
use schemaparser::{SchemaTemplate};
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

type Version = String;

// Defines the types allowed in the definition of a schema
enum AttributeType {
    Str,
    Bool,
    Array,
    Int,
    Float
}

struct Attribute {
    name: &str, 
    value_type: AttributeType,
}
impl Serialize for Attribute {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer, {
                let mut s= serializer.serialize_struct("Attribute", 2)?;
                s.serialize_field("name", &self.name)?;
                s.serialize_field("value_type", &self.value_type)?;
                s.end()
            }
}

struct Component {
    name: &str,
    attributes: &[Attribute]
}
impl Serialize for Component {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> 
    where
    S: Serializer 
    {
        let mut s = serializer.serialize_struct("Component", 2)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("attributes", &self.attributes)?;
        s.end()
    }
}

struct Metadata {
    required: bool,
    attributes: HashMap<&str, Attribute>
}
impl Serialize for Metadata {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer {
            let mut s = serializer.serialize_struct("Metadata", 2)?;
            s.serialize_field("required", &self.required)?;
            s.serialize_field("attributes", &self.attributes)?;
            s.end()
        }
}
struct Resolver {
    name: &str,
}
impl Serialize for Resolver {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> 
    where 
    S: Serializer {
        let mut s = serializer.serialize_struct("Resolver", 1)?;
        s.serialize_field("name", &self.name)?;
        s.end()
    }
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
