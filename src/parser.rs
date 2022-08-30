mod configparser;
mod treeparser;

// Parser Module
// Receives the yaml configuration file and parses it to create the internal state structure of
// the program. It conducts the semantic analysis of the passed configuration before creating the
// program tree with the nodes responsible for execution.
// Functionality:
// * Check and validate template version
// * Check and validate template type
// * Parse Endpoint section
// * Parse Params
// * Parse Loaders
 
mod configparser;

// Traits
trait Parse {
    /// Parse configuration blocks from a passed RawConfig into a parseTree representing a program tree
    /// with resolved execution blocks.
    fn parse(config: configparser::RawConfig) -> configparser::ParseTree {}

}

/// Validate configuration parse types
trait Validate: Parse {}

// Struct definitions

// Parses an Endpoint config element into a GraphNode
pub struct EndpointParser {}
impl EndpointParser {
   pub fn new() -> Self {
        EndpointParser {
        }
   } 
}

pub struct ParamParser {}
impl ParamParser {
   pub fn new() -> Self {
        ParamParser {
        }
   } 
}


pub struct LoaderParser {}
impl LoaderParser {
   pub fn new() -> Self {
        LoaderParser {
        }
   } 
}
