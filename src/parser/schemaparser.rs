use std::collections::HashMap;
use schema::{Version};
//Parses a schema configuration file into a schema objecect usable within the system

struct SchemaTemplate {
    version: Version,
    components: HashMap,
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
    filebuffer: &[u8],

}
impl SchemaParser {
    pub fn new() -> Self {
        SchemaParser {
        }
    }
    pub fn parse(&mut self, file: &str) -> SchemaTemplate {
       unimplemented!();
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
    fn parse_schema() {
        let parser =SchemaParser::new();
        let schema: SchemaTemplate = parser.parse("../../data/teschema_v1.yaml");

    }
}
