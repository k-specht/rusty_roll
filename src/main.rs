use std::fs::{ File, metadata, OpenOptions };
use std::io::{stdin, Read, Write, Result, prelude::* };
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

                    // If file exists, prompt to quit if it is larger than 10MB
                    let meta = metadata(filename).expect("File not found.");
                    let mut proceed = true;
                    if meta.len() > 10000000 {
                        println!("WARNING! This file is over 10 Megabytes. You should probably compress it.");
                        print!("Continue anyway? (Building this could take well over an hour) [Y/N]:");
                        
                        let stdin = stdin();
                        let inp = stdin.lock().lines().last();
                        match inp {
                            Some(x) => {
                                let input = x.unwrap().to_uppercase();
                                proceed = input == "Y" || input == "YES";
                            }
                            None    => {
                                panic!("Nothing found in user input, quitting...");
                            }
                        }
                    }

                    // Reads the file & sets up crate
                    if proceed {
                        // Read the file into a byte Vector
                        let output = to_byte_array(filename.to_string());

                        // Copy video.rs to a new crate
                        std::fs::create_dir_all("bin").expect("Error creating directory \"bin\".");
                        std::fs::copy("video.rs", "bin/main.rs").expect("Failed to copy video.rs to bin directory.");

                        // Format the output to a new module & write to the new crate
                        let func_calls = create_module("bin/roll.rs", output, filename);

                        // Configure video.rs to use the module by appending the functions
                        append_rolls("bin/main.rs", func_calls);
                        
                        println!("Video-embedded binary crate created!\nUse \"rustyroll --run <crate_name>\" to build it, or build it yourself.");
                    }
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

    let _bytes = f.read_to_end(&mut output).unwrap();
    //println!("Read {} bytes into a Vector.", bytes);

    output
}

// Converts the given byte Vector into a byte array String & the number of elements
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
fn _read_file(filename: &str) -> Result<String> {
        
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

fn append_rolls(filename: &str, num: usize) {
    // Sets up a custom function that contains generated function calls
    let mut out = String::new();
    out += "\nmod roll;\n";
    out += "//Saves the video to the local directory\n";
    out += "fn save_video(filename: &str) {\n";

    // Creates calls for the auto-generated functions
    for i in 0..num {
        out += "    file.write_all(&roll::roll::roll";
        out += &i.to_string();
        out += "roll()).unwrap();\n";
    }

    out += "\n";

    // Append the new calls to the specified file
    let mut f = OpenOptions::new()
        .append(true)
        .open(filename)
        .expect("Failed to open the new main.rs file.");

    f.write_all(out.as_bytes()).expect("Failed to append to the file!");
}

// Writes a module to the specified file path and returns the number of methods
fn create_module(filename: &str, data: Vec<u8>, custom_name: &str) -> usize {
        
    // Open file in write mode
    //let mut file = File::create(filename).unwrap();
    let mut f = OpenOptions::new()
        .append(true)
        .read(true)
        .create(true)
        .open(filename)
        .expect("Failed to open file.");

    // Split data into arrays of size 30k or less
    let mut dataset: Vec<Vec<u8>> = Vec::new();
    let mut index = 0;
    while index < data.len() {
        // Only read as long as data exists
        let read_until = std::cmp::min(index + 30000, data.len());
        let mut subset: Vec<u8> = Vec::new();

        // Adds the data to the subset
        while index < read_until {
            subset.push(data[index]);
            index += 1;
        }

        dataset.push(subset);
    } // Possible infinite loop if index doesn't reach data.len() in sub-loop?

    // Builds the module
    let mut out_string = String::new();
    let mut fn_num: usize = 0;
    println!("Converting data into Rust module...");
    out_string += "pub mod roll {\n";

    // Customizes executable filename
    out_string += "pub fn name() -> String { ";
    out_string += custom_name;
    out_string += " }\n";

    // Adds functions to the module
    for set in dataset {
        // Get the subset's size & array form
        let (array_form, size) = vec_to_string(set);

        // Builds the function signature
        out_string += "pub fn roll";
        out_string += &fn_num.to_string();
        out_string += "() -> [u8; ";
        out_string += &size.to_string();
        out_string += "] {\n";

        // Fills the data into the function and closes it
        out_string += &array_form;
        out_string += "\n}\n";

        // Ensures each method has a unique & serialized name
        fn_num     += 1;
    }
    out_string += "}\n";

    // Write data to file
    //let (vec_data, _size) = vec_to_string(data);
    println!("Done! Writing data to {}.", filename);
    f.write_all(out_string.as_bytes()).unwrap();
    fn_num
}