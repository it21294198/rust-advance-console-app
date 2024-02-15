#![allow(unused_assignments)]
// Suppress the Warnings: ⬆︎

// !trait is similer to Interfaces
trait  Car {
    fn drive(&self);
}
trait  Boat {
    fn paddle(&self){
        println!("Boat can be paddle");
    }
}
struct Toyota;
impl Car for Toyota {
    fn drive(&self) {
        println!("Toyota is a best car");
    }
}

trait Amphibious:Car+Boat {}
struct Hovercraft;
impl Amphibious for Hovercraft {}
impl Car for Hovercraft {
    fn drive(&self) {
        println!("Hover craft drives");
    }
}
impl Boat for Hovercraft {}

struct Train;
impl Train {
    fn ride(&self){
        println!("Train is ride");
    }
}

fn to_office(vehicle: &Train){
    vehicle.ride();
}
fn to_trip_dynamic(vehicle:&dyn Car){
    vehicle.drive();
}
// ! better to use impl insted of dyn
fn to_trip_static(vehicle:&impl Car){
    vehicle.drive();
}

fn go_on_swamp(vehicle: &impl Amphibious){
    vehicle.drive();
    vehicle.paddle();
}

fn colorize_text(text: &str, font_size:i32,color_code: u8) -> String {
    // *println!("\x1b[92m{}\x1b[0m", error); //will output this
    format!("\x1b[{};{}m{}\x1b[0m", font_size,color_code, text)
}

// !import features fille
// --import-->>> pub mod features; ---use-->>> features::feature_test();
// or
mod features;

// use features::feature_test;
use features::{feature_test,get_cli_inputs,fetch_data_from_api};


fn main() {

    let error = "Error: Something went wrong!";
    println!("{}", colorize_text(error, 3,91));
    println!("\x1b[1;92m{}\x1b[0m", error);

    // Underlined green color for the error message
    println!("\x1b[4;92m{}\x1b[0m", error);

    // Bold and underlined green color for the error message
    println!("\x1b[1;4;92m{}\x1b[0m", error);

    // Simulating italic font using Unicode characters
    println!("𝘛𝘩𝘪𝘴 𝘪𝘴 𝘢 𝘣𝘰𝘭𝘥 𝘦𝘳𝘳𝘰𝘳: {}", error);
    
    // Simulating monospace font using Unicode characters
    println!("𝙏𝙝𝙞𝙨 𝙞𝙨 𝙖 𝙢𝙤𝙣𝙤𝙨𝙥𝙖𝙘𝙚 𝙚𝙧𝙧𝙤𝙧: {}", error);

    // let mut cli = vec![];
    // match get_cli_inputs(){
    //     Ok(input)=>{
    //         cli = input;
    //     }
    //     Err(err)=> {
    //         println!("{:?}",err);
    //         return;
    //     }
    // }
    // for i in cli.iter(){
    //     println!("{}",i);
    // }

    println!("Initial");
    let vehicle = Train;
    to_office(&vehicle);

    let car = Toyota;
    to_trip_dynamic(&car);
    to_trip_static(&car);
    
    let hover = Hovercraft;
    go_on_swamp(&hover);
    // !call features_test function inside features file
    // feature_test();
    match fetch_data_from_api() {
        Ok(data) => {
            println!("Data from API:\n{:?}", data);
        }
        Err(err) => {
            eprintln!("Error fetching data from API: {}", err);
        }
    }


}

// !Sample waintig function
// use std::thread;
// use std::time::{Duration};
// use std::io::{self, Write};
// use std::sync::atomic::{AtomicBool, Ordering};
// use std::sync::Arc;
// use std::process;

// fn main() {
    //     // Print colored error message
    //     let error = "Error: Something went wrong!";
    //     println!("\x1b[1;92m{}\x1b[0m", error);
    
    //     // Create a flag to signal the loading animation thread to terminate
    //     let running = Arc::new(AtomicBool::new(true));
    //     let running_clone = Arc::clone(&running);
    
    //     // Start the loading animation thread
    //     let handle = thread::spawn(move || {
        //         let loading_symbols = ["▉▉▉▉▉", "▊▊▊▊▊", "▋▋▋▋▋", "▌▌▌▌▌", "▍▍▍▍▍", "▎▎▎▎▎", "▏▏▏▏▏"];
        //         let loading_symbols = ["↱", "↴", "↲", "↱"];
        //         let loading_symbols = ["⤿", "⤻", "⤺", "↶"];
        //         let loading_symbols = ["▁", "▂", "▃", "▄", "▅", "▆", "▇", "█"];
        //         let delay = Duration::from_millis(50);
        //         while running_clone.load(Ordering::Relaxed) {
//             for symbol in &loading_symbols {
//                 print!("{}\r", symbol);
//                 io::stdout().flush().unwrap();
//                 thread::sleep(delay);
//             }
//         }
//     });

//     // Wait for 5 seconds in the main thread
//     thread::sleep(Duration::from_secs(5));

//     // Set the flag to signal the loading animation thread to terminate
//     running.store(false, Ordering::Relaxed);

//     // Join the loading animation thread
//     handle.join().unwrap();

//     // Terminate the program
//     process::exit(0);
// }

// !Sample waintig pogram 2
// use std::thread;
// use std::time::{Duration};
// use std::io::{self, Write};
// use std::sync::atomic::{AtomicUsize, Ordering};
// use std::sync::Arc;

// // Example function to simulate data fetching from an API
// fn fetch_data_from_api1() -> Result<String, String> {
//     // Simulate API request delay
//     thread::sleep(Duration::from_secs(2));
//     Ok("Data fetched from API 1".to_string())
// }

// // Example function to simulate data fetching from another API
// fn fetch_data_from_api2() -> Result<String, String> {
//     // Simulate API request delay
//     thread::sleep(Duration::from_secs(3));
//     Ok("Data fetched from API 2".to_string())
// }

// // Function to display loading animation
// fn display_loading_animation(running: Arc<AtomicUsize>) -> thread::JoinHandle<()> {
//     thread::spawn(move || {
//         let loading_symbols = ["▉▉▉▉▉", "▊▊▊▊▊", "▋▋▋▋▋", "▌▌▌▌▌", "▍▍▍▍▍", "▎▎▎▎▎", "▏▏▏▏▏"];
//         let delay = Duration::from_millis(250);
//         while running.load(Ordering::Relaxed) > 0 {
//             for symbol in &loading_symbols {
//                 print!("{}\r", symbol);
//                 io::stdout().flush().unwrap();
//                 thread::sleep(delay);
//             }
//         }
//     })
// }

// fn main() {
//     // Create a counter to keep track of active data fetching operations
//     let running = Arc::new(AtomicUsize::new(0));

//     // Start the loading animation thread
//     let handle = display_loading_animation(Arc::clone(&running));

//     // Increment the counter before starting each data fetching operation
//     running.fetch_add(1, Ordering::Relaxed);

//     // Fetch data from API 1
//     let running_clone1 = Arc::clone(&running);
//     thread::spawn(move || {
//         match fetch_data_from_api1() {
//             Ok(data) => {
//                 println!("{}", data);
//             }
//             Err(err) => {
//                 println!("Error fetching data from API 1: {}", err);
//             }
//         }
//         // Decrement the counter after completing the data fetching operation
//         running_clone1.fetch_sub(1, Ordering::Relaxed);
//     });

//     // Increment the counter before starting the next data fetching operation
//     running.fetch_add(1, Ordering::Relaxed);

//     // Fetch data from API 2
//     let running_clone2 = Arc::clone(&running);
//     thread::spawn(move || {
//         match fetch_data_from_api2() {
//             Ok(data) => {
//                 println!("{}", data);
//             }
//             Err(err) => {
//                 println!("Error fetching data from API 2: {}", err);
//             }
//         }
//         // Decrement the counter after completing the data fetching operation
//         running_clone2.fetch_sub(1, Ordering::Relaxed);
//     });

//     // Wait for all data fetching operations to complete
//     while running.load(Ordering::Relaxed) > 0 {
//         thread::sleep(Duration::from_millis(100));
//     }

//     // Stop the loading animation
//     running.store(0, Ordering::Relaxed);
    
//     // Join the loading animation thread
//     handle.join().unwrap();
// }
