mod parser;
mod load;
mod cmdln;

fn main() {
    // Establish a connection to the cli to receive instructions
    let arguments = cmdln::connect_to_cli(); // Creates app and connects it to the CLI input
    println!("{:#?}", arguments);

    // Establish the schema to be used for parsing
    // 1. Instantiate a schema based on version name
    
    //Parse the provided template
    //Validate the parsed template works with the set schema version

    //Parse the template to internal representation (State)
    //Store the updates to the internal state 
    //
}




