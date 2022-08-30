// The ArgParser receives arguments from either the command
// line or another input source and parses it into a dictionary
// of cleaned arguments to use within the application.
// ```
// let parser = ArgParser::new();
// ```
use std::env::Args;
use std::collections::HashMap;


#[derive(Debug)]
pub struct ArgParser{
    input: Option<String>,
    arguments: HashMap<String, String>,
}
impl ArgParser {
    pub fn new() -> Self {
        ArgParser {
            input: None,
            arguments: HashMap::new(),
        }       
    }
    /// Parses the arguments from StdIn into an internal 
    /// argument dict
    pub fn parse(&self, args: Args) -> () {
        //parse the incoming arguments
        println!("{:?}", args);
        //place them into the self.arguments HashMap
    }
    

}


#[cfg(test)]
mod tests {

    use super::*; 
    
    #[test]
    fn can_instantiate() {
    }

    #[test]
    fn accepts_cli_args() {
        assert!(false);
    }

    #[test]
    fn parses_string_args_from_cli() {
        assert!(false);
    }
    


}

