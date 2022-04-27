/* Note!
    This file contains the runtime binary methods, not the builder code.
    Be careful if you modify anything, especially after the auto-generated section!
*/
use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;

/// Entry point for the executable that gets built.
fn main() {
    // Saves the video to the local directory (fails if no write perms)
    let filename = roll::roll::name();
    println!("Starting rusty_roll with filename: {}", &filename);
    save_video(&filename);
    open_media_player(&filename);
}

/// Uses the platform's media player to open a video.
fn open_media_player(filename: &str) {
    // On Linux sh, adding the args individually after is buggy
    let command = format!("xdg-open {}", filename.clone());
    let mut output = if cfg!(target_os = "windows") {
        Command::new("cmd")
                    .args(&["/C", "start", "mswindowsvideo:", "ofe|help|", filename.clone()])
                    .status()
                    .expect("command failed")
    } else {
        Command::new("sh")
                    .args(&["-c", &command])
                    .status()
                    .expect("command failed")
    };

    // If it failed to open the default app, it tries to find VLC media player
    if !output.success() && cfg!(target_os = "windows") {
        output = Command::new("cmd")
                .args(&["/C", "\"C:\\Program Files\\VideoLAN\\VLC\\vlc.exe\"", filename.clone()])
                .status()
                .expect("command failed");
    } else if !output.success() && cfg!(target_os = "linux") {
        output = Command::new("sh")
                .args(&["-c", "vlc", filename.clone()])
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