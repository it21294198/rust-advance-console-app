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

// !achieve HTTP requests in Rust using the standard library's std::net::TcpStream.
use std::io::{self, Read, Write};
use std::net::TcpStream;

#[derive(Debug)]
pub struct MyObject {
    pub id: u32,
    pub title: String,
}

pub fn fetch_data_from_api() -> io::Result<MyObject> {
    // Open a TCP connection to the API server
    let mut stream = TcpStream::connect("jsonplaceholder.typicode.com:80")?;

    // Send a GET request
    let request = "GET /posts/1 HTTP/1.1\r\nHost: jsonplaceholder.typicode.com\r\nConnection: close\r\n\r\n";
    stream.write_all(request.as_bytes())?;

    // Read the response
    let mut response = String::new();
    stream.read_to_string(&mut response)?;

    // Manually parse JSON response
    let id_start_index = response.find("\"id\":").ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "ID not found in response"))?;
    let title_start_index = response.find("\"title\":").ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Title not found in response"))?;
    let id_end_index = response[id_start_index..].find(',').ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid ID format in response"))? + id_start_index;
    let title_end_index = response[title_start_index..].find(',').ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid title format in response"))? + title_start_index;
    
    let id_str = &response[id_start_index + 6..id_end_index];
    let title_str = &response[title_start_index + 9..title_end_index - 1];
    
    let id = id_str.parse::<u32>().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Failed to parse ID"))?;
    
    Ok(MyObject {
        id,
        title: title_str.to_string(),
        // Initialize other fields as needed
    })
}

// !sample fetch
// pub fn fetch_data_from_api() -> io::Result<String> {
//     // Open a TCP connection to the API server
//     let mut stream = TcpStream::connect("jsonplaceholder.typicode.com:80")?;

//     // Send a GET request
//     let request = "GET /posts/1 HTTP/1.1\r\nHost: jsonplaceholder.typicode.com\r\nConnection: close\r\n\r\n";
//     stream.write_all(request.as_bytes())?;

//     // Read the response
//     let mut response = String::new();
//     stream.read_to_string(&mut response)?;

//     Ok(response)
// }
