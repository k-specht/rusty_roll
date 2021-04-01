use std::fs::{ File, OpenOptions };
use std::io::{Read, Write, Result };
use std::fmt::Display;
//use std::process::Command;
use std::env;

// Uses the command-line arguments to manage a new crate
fn main() {
    // Command-line args determine behavior of the builder
    let args: Vec<String> = env::args().collect();

    // If there are arguments, match them against allowed behavior
    if args.len() > 0 {
        match args[0].as_str() {
            // Lists supported args
            "--help" | "-h" => {
                println!("Supported args:\n-h / --help = Lists args & usage.");
                println!("-b / --build = Builds a loader for the specified file.\nUsage: rustyroll -b \"somefile.mp4\"");
            }

            // Builds methods for a crate that loads that video
            "--build" | "-b" => {
                if args.len() > 1 {
                    let filename = &args[1];
                    // Try finding the file
                    let _check = read_file(filename).unwrap();

                    // If it exists, prompt to quit if it is larger than 10MB

                    // Read the file
                    let output = to_byte_array(filename.to_string());

                    // Copy video.rs to a new crate

                    // Format the output to a new module & write to the new crate
                    write_file("output.txt", output);

                    // Configure video.rs to use the module
                    
                    // Rename video.rs -> main.rs and quit with a success message

                    // If it doesn't exist, print an error and quit
                } else {
                    println!("ERR: Must specify file name of .mp4 file.");
                }

            }

            // Try to build the new crate
            "-r" | "--run" => {
                if args.len() > 1 {
                    let _crate_name = &args[1];

                    // Try finding the crate

                    // If it exists, confirm that the user wants to build it (takes half an hour)

                    // Build the crate

                    // Quit with a success or fail message
                }
            }

            // Unsupported arguments
            _ => {
                println!("ERR: Unsupported argument entered: {}.\nUse \"rustyroll -h\" to get a list of supported args.", args[0]);
            }
        }
    } else {
        println!("ERR: Must specify command line arguments.\nEx. rustyroll --help");
    }
}

// Converts the given file into a byte Vector
fn to_byte_array(filename: String) -> Vec<u8> {
    let mut output = Vec::new();

    let mut f = File::open(filename).unwrap();

    let bytes = f.read_to_end(&mut output).unwrap();
    println!("Read {} bytes into a Vector.", bytes);

    output
}

// Converts the given byte Vector into a byte array String & the number of elements
// TODO: Change this into a vec_to_function() function instead
fn vec_to_string<T>(data: Vec<T>) -> (String, usize) 
where T: Display {
    let mut output = String::new();
    output += "[\n    ";
    let mut count = 0;
    let size = data.len();
    for thing in data {
        output += &thing.to_string();
        output += if count % 10 == 0 {",\n    "} else {", "};
        count += 1;
    }
    output += "]";
    (output, size)
}
    
// Reads the specified file or creates an empty one and reads that
fn read_file(filename: &str) -> Result<String> {
        
    // Open file
    let mut file = OpenOptions::new()
        .append(true)
        .read(true)
        .create(true)
        .open(filename)
        .expect("Failed to open file.");

    // Read file into buffer
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    Ok(buffer)
}

// Writes to the specified file
fn write_file(filename: &str, data: Vec<u8>) {
        
    // Open file in write mode
    //let mut file = File::create(filename).unwrap();
    let mut f = OpenOptions::new()
        .append(true)
        .create(true)
        .open(filename)
        .expect("Failed to open file.");

    // Write data to file
    let (vec_data, size) = vec_to_string(data);
    f.write_all(vec_data.as_bytes()).unwrap();
}