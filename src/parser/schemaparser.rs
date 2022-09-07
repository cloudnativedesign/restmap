use std::fs;
use std::collections::HashMap;
use schema::{Version, AttributeType, Component, Metadata, Attribute, Resolver};
//Parses a schema configuration file into a schema objecect usable within the system

pub struct SchemaTemplate {
    version: Version,
    components: &[Component],
    resolvers: &[Resolver],
}

impl SchemaTemplate {
    pub fn new(version: &str) -> Self {
        let version: Version = version.to_string();
        let components = HashMap::new();
        SchemaTemplate {
            version,
            components,
        }
    }
}

///Reads a schema from file and parses it into a SchemaTemplate usable to load into a schema for
///valdiation and graph compilation
struct SchemaParser {

}

impl SchemaParser {
    pub fn new() -> Self {
        SchemaParser {
        }
    }
    ///Parse a schema template yaml file
    pub fn parse(&mut self, file_path: &str) -> SchemaTemplate {
        // Read yaml file
        let contents = fs::read_to_string(file_path)
            .expect("File not found at {}", file_path);

        // Parse out list of components
        // Parse out list of attributes per component
        // Create and return SchemaTemplate
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instantiate_schemaparser() {
        let parser = SchemaParser::new();
    }
    #[test]
    fn instantiate_schematemplate() {
        let template_v1 = SchemaTemplate::new("0.1.0");
    }
    #[test]
    fn load_file() {
        let parser = SchemaParser::new();
        let content = parser.load_file("../../data/schemata/2022-01-1.yaml");
    }
    #[test]
    fn parse_schema() {
        let parser =SchemaParser::new();
        let schema: SchemaTemplate = parser.parse("../../data/teschema_v1.yaml");

    }
}
