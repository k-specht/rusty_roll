# rusty_roll
An odd Rust program for creating video-embedded binaries. Great for awful memes.

Using Rusty Roll
-----------------------------------------------------------------------------------
1. Clone the repo
2. Run "cargo build --release"
3. Save a video file to the project directory
4. Run "cargo build --release -- --build <filename.mp4>"
5. Run "cargo build --release -- --run"
Note: The program will take a long time on this step.
6. Copy the resulting binary from bin/target/release


Tips:

Use "rusty_roll --help" to get more information on available commands.

Make sure your video file is small! The program will warn you about 
any files you try to use >10 Megabytes, since they will take a while to 
build into the binary.

Rustc may use a lot of RAM when trying to build the resulting binary file.
Make sure you have enough RAM, or use a very small video file (~2-3 MB).

Need help compressing a video? Want to apply random filters to a video?! Check out VideoLAN's VLC Media Player!
It can play just about any video file, compress them, change the encoding and more.
https://github.com/videolan/vlc 


Note:

This project can be used to store & save virtually any file into an executable.
However, there are typically far better ways to do so; this project is just a 
proof of concept.

PS. Meme your friends! :)
