use std::fs;
use serde_yaml;
use std::collections::HashMap;
use schema::{Version, AttributeType, Component, Metadata, Attribute, Resolver};
//Parses a schema configuration file into a schema objecect usable within the system

pub struct SchemaTemplate<'a> {
    version: Version,
    components: &'a[Component],
    resolvers: &'a[Resolver],
}

impl<'a> SchemaTemplate<'a> {
    pub fn new(version: &str) -> Self {
        let version: Version = version.to_string();
        let components = HashMap::new();
        let resolvers = HashMap::new();
        SchemaTemplate {
            version,
            components,
            resolvers,
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
    pub fn parse(&mut self, file_path: &str) -> Result<SchemaTemplate, serde_yaml::Error> {
        // Read yaml file
        let contents = self.load_file(file_path);
        // Parse out list of components
        let config: serde_yaml::Value = serde_yaml::from_str(&contents)?;
        // Parse out list of attributes per component
        // Create and return SchemaTemplate
        

        Ok(SchemaTemplate {
            version: config["version"],
            components: config["components"],
            resolvers: config["resolvers"],
        })
    }
    pub fn load_file(&self, file_path: &str) -> String {
        fs::read_to_string(&file_path)
            .expect("File not found at {}")
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
