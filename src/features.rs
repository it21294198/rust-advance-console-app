pub fn feature_test(){
    println!("Test ran");
}

/*
Ok(Vec<String>): Represents a successful operation, 
?returning a vector of strings (command-line arguments).
!Err(&'static str): Represents a failed operation, 
?returning a string slice (error message) with a static lifetime.
*/

/*
you could associate 'static with "static memory" or "program lifetime" 
and 'static str with "static string slice" or "string literal with 
static lifetime".
*/
use std::env;
pub fn get_cli_inputs()->Result<Vec<String>,&'static str>{
    let cli_input:Vec<String> = env::args().collect(); 
    if cli_input.len() <= 1 {
        println!("cargo run -- [args]");
        return  Err("No args founed");
    }
    Ok(cli_input)
}