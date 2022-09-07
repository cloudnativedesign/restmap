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
