use std::fs::File;
use std::io::prelude::*;
use serde_yaml;
use super::schema::{Version, SchemaTemplate};
//Parses a schema configuration file into a schema objecect usable within the system

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
        let contents = self.load_file(file_path).expect("Unable to open file");
        // Create and return SchemaTemplate
        let config: SchemaTemplate = serde_yaml::from_str(&contents)?;
        Ok(config)
    }
    ///Reads a file into a string buffer
    pub fn load_file(&self, file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
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
        let schema: SchemaTemplate = parser.parse("./data/schemata/2022-01-01.yaml").unwrap();
    }
}
