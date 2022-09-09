use std::fs::File;
use std::io::prelude::*;
use serde_yaml;
use super::schema::{Version, SchemaTemplate};
//Parses a schema configuration file into a schema objecect usable within the system

///Reads a schema from file and parses it into a SchemaTemplate usable to load into a schema for
///valdiation and graph compilation
///
struct Schema{}

pub struct SchemaParser {
    pub schema_version: String,
    pub schema_template: Option<SchemaTemplate>,
}

impl SchemaParser {
    pub fn new() -> Self {
        SchemaParser {
            schema_version: String::new(),
            schema_template: None,
        }
    }
    /// Attempts loading a provided schema a specified file location. Fails if schema is not
    /// aligned with the initalized SchemaVersion used
    pub fn load_schema(&self, schema: Schema) -> Result<Schema, Box<dyn std::error::Error>> {
        unimplemented!();
    }
    ///Configures the parser with a given schema version
    pub fn initialize_schema(&mut self, schema_template_file: &str) ->  Result<(), serde_yaml::Error> {
        let content = self.load_file(&schema_template_file).expect("something");
        let schema_template = self.parse_template(&content)?;
        self.schema_version = schema_template.version.to_owned();
        self.schema_template = Some(schema_template);
        Ok(())
    }
    ///Parse a schema template yaml file
    fn parse_template(&mut self, file_path: &str) -> Result<SchemaTemplate, serde_yaml::Error> {
        // Read yaml file
        let contents = self.load_file(file_path).expect("Unable to open file");
        // Create and return SchemaTemplate
        let config: SchemaTemplate = serde_yaml::from_str(&contents)?;
        Ok(config)
    }
    ///Reads a file into a string buffer
    fn load_file(&self, file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut file = File::open(&file_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
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
    fn load_file() {
        let parser = SchemaParser::new();
        let content = parser.load_file("../../data/schemata/2022-01-1.yaml");
    }
    #[test]
    fn parse_schema() {
        let mut  parser =SchemaParser::new();
        parser.parse_template("./data/schemata/2022-01-01.yaml").unwrap();
    }
    #[test]
    fn initialize_schema() {
        let mut parser =SchemaParser::new();
        parser.initialize_schema("./data/schemata/2022-01-01.yaml").expect("File was not available");
        assert_eq!(*parser.schema_version, "2022-01-01".to_owned())
    }

}
