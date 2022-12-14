pub mod component;
pub mod attribute;
pub mod schema;
pub mod schemaparser;
pub mod configparser;

// Parser Module
// Receives the yaml configuration file and parses it to create the internal state structure of
// the program. It conducts the semantic analysis of the passed configuration before creating the program tree with the nodes responsible for execution.
// Functionality:
// * Check and validate template version
// * Check and validate template type
// * Parse Endpoint section
// * Parse Params
// * Parse Resolvers

// Traits
trait Parse<T> {
/// Parse configuration blocks from a passed RawConfig into a parseTree representing a program tree
/// with resolved execution blocks.
fn parse(&self, config: configparser::RawConfig) {}

}

/// Validate configuration parse types
trait Validate<T> : Parse<T> {}

// Struct definitions

// Parses an Endpoint config element into a GraphNode
pub struct EndpointParser {}
impl EndpointParser {
pub fn new() -> Self {
    EndpointParser {
    }
} 
}
impl<T> Parse<T> for EndpointParser {
    fn parse(&self, config: configparser::RawConfig) {
        unimplemented!();
    }
}

pub struct ParamParser {}
impl ParamParser {
pub fn new() -> Self {
    ParamParser {
    }
} 
}

pub struct ResolverParser {}
impl ResolverParser {
pub fn new() -> Self {
    ResolverParser {
    }
} 
}
impl<T> Parse<T> for ResolverParser {

    fn parse(&self, config: configparser::RawConfig){
        unimplemented!(); 
    }

}

