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
                .args(&["/C", "\"C:\\Program Files\\VideoLAN\\VLC\\vlc.exe\"", filename])
                //.arg("exit")
                .status()
                .expect("command failed")
    } else {
        Command::new("sh")
                .args(&["-c", "vlc", filename])
                .status()
                .expect("command failed")
    };

    // If it failed to open VLC, Windows 10 has the Movies & TV app! You can't escape the rick roll :)
    if !output.success() && cfg!(target_os = "windows") {
        output = Command::new("cmd")
                    .args(&["/C", "start", "mswindowsvideo:", "ofe|help|", filename])
                    .status()
                    .expect("command failed");
    } else if !output.success() && cfg!(target_os = "linux") {
        output = Command::new("sh")
                    .args(&["-c", "xdg-open", filename])
                    .status()
                    .expect("command failed");
    }

    if output.success() {
        println!("Haha, gotcha! :)");
    } else {
        println!("Aw... try opening the file yourself anyway?");
    }
}

// (Warning! Auto-generated! Don't type anything after this!)
// %AUTO% //