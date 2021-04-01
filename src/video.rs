// Note! This file contains the runtime binary methods, not the builder code.
// Be careful if you modify anything, especially after the auto-generated section!
use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;

// Main driver for the executable that gets built
fn main() {
    // Saves the video to the local directory (fails if no write perms)
    let filename = roll::roll::name();
    save_video(&filename);
    open_media_player(&filename);
}

// Uses the platform's media player to open a video
fn open_media_player(filename: &str) {
    
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