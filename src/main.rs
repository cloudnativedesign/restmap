mod authenticate;
mod cmdln;
mod load;
mod parser;

fn main() {
    //  
    let arguments = cmdln::connect_to_cli(); // Creates app and connects it to the CLI input
    println!("{:#?}", arguments);
}


