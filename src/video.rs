// Note! This file contains the runtime binary methods, not the builder code.
// Be careful if you modify anything, especially after the auto-generated section!
use std::fs::{ File, OpenOptions };
use std::io::{Read, Write };
use std::process::Command;

// Main driver for the executable that gets built
fn main() {
    // Saves the video to the local directory (fails if no write perms)
    let filename = roll::name();
    save_video(filename);
    open_media_player(filename);
}

// Uses the platform's media player to open a video
fn open_media_player(filename: String) {
    
    // Attempts to open VLC
    println!("Starting VLC media player...");
    let mut output = if cfg!(target_os = "windows") {
        //Command::new("\"C:\\Program Files\\VideoLAN\\VLC\\vlc.exe\"")
        Command::new("cmd")
                //.args(&["/C", "echo hello"])
                .args(&["/C", "C:\\Program Files\\VideoLAN\\VLC\\vlc.exe", filename])
                //.arg("exit")
                .status()
                .expect("Failed to start VLC! Try opening the generated file yourself :)")
    } else {
        Command::new("sh")
                .args(&["-c", "vlc", filename])
                .status()
                .expect("Failed to start VLC! If you have a different media player, try using that instead.")
    };

    // If it failed to open VLC, Windows 10 has the Movies & TV app! You can't escape the rick roll :)
    if !output.success() && cfg!(target_os = "windows") {
        output = Command::new("cmd")
                    .args(&["/C", "start", "mswindowsvideo:", "ofe|help|", filename])
                    .status()
                    .expect("Failed to open VLC and Windows Movies & TV. Try opening the file manually.")
    }

    if output.success() {
        println!("Haha, gotcha! You cannot escape the rick rolling. :)");
    } else {
        println!("Darn, you must be using some old Windows version, or never installed VLC on linux.\nI tried!");
    }
}

// (Warning! Auto-generated! Don't type anything after this!)
// %AUTO% //

mod roll;

// Saves the video to the local directory
fn save_video(filename: &str) {
    //let res = roll::roll::rolly();

    let mut file = File::create(filename).unwrap();
    file.write_all(&roll::roll::roll()).unwrap();
    file.write_all(&roll::roll::roll2()).unwrap();
    file.write_all(&roll::roll::roll3()).unwrap();
    file.write_all(&roll::roll::roll4()).unwrap();
    file.write_all(&roll::roll::roll5()).unwrap();
    file.write_all(&roll::roll::roll6()).unwrap();
    file.write_all(&roll::roll::roll7()).unwrap();
    file.write_all(&roll::roll::roll8()).unwrap();
    file.write_all(&roll::roll::roll9()).unwrap();
    file.write_all(&roll::roll::roll10()).unwrap();
    file.write_all(&roll::roll::roll11()).unwrap();
    file.write_all(&roll::roll::roll12()).unwrap();
    file.write_all(&roll::roll::roll13()).unwrap();
    file.write_all(&roll::roll::roll14()).unwrap();
    file.write_all(&roll::roll::roll15()).unwrap();
    file.write_all(&roll::roll::roll16()).unwrap();
    file.write_all(&roll::roll::roll17()).unwrap();
    file.write_all(&roll::roll::roll18()).unwrap();
    file.write_all(&roll::roll::roll19()).unwrap();
    file.write_all(&roll::roll::roll20()).unwrap();
    file.write_all(&roll::roll::roll21()).unwrap();
    file.write_all(&roll::roll::roll22()).unwrap();
    file.write_all(&roll::roll::roll23()).unwrap();
    file.write_all(&roll::roll::roll24()).unwrap();
    file.write_all(&roll::roll::roll25()).unwrap();
    file.write_all(&roll::roll::roll26()).unwrap();
}
