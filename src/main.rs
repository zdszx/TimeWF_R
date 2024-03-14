use fltk::{app::*, button::*, prelude::*, image::*,enums::*,window::*};
use fltk::{app, frame::Frame, image::SharedImage, prelude::*, window::Window};
use std::error::Error;

use std::sync::{Arc, Mutex};
use async_std::task;
use std::{fs::File, io::BufReader};
use rodio::{Decoder, OutputStream, source::Source,Sink};
use std::env;
use std::thread;
use std::time::Duration;
use tokio::time::interval;
mod time_util;

#[tokio::main]
 async fn main() {
    let app = App::default().with_scheme(app::Scheme::Gleam);
    let mut wind = Window::new(100, 100, 500, 800, "Async FLTK Example");
    let mut button = Button::new(150, 150, 100, 50, "Press me!");
    button.set_color(Color::from_rgb(0, 180, 255)); // Set button color
    button.set_label_color(Color::White); // Set label color

    let mut frame = Frame::default_fill();
    let image_result = SharedImage::load("./pic/setpainting_0.png");
    match image_result {
        Ok(image) => {
            frame.set_image(Some(image));
        }
        Err(e) => {
            eprintln!("Failed to load image: {}", e);
        }
    }

    let volume: Arc<Mutex<f32>> = Arc::new(Mutex::new(0.5)); // Default volume

    button.set_callback({
        let volume = Arc::clone(&volume);
        move |_| {
            let mut volume = volume.lock().unwrap(); // Lock the mutex
            *volume += 0.1; // Increase volume by 0.1
            if *volume > 5.0 {
                *volume = 1.0; // Cap volume at 1.0
            }
            let volume = *volume; // Clone volume for async block
           // drop(volume); // Drop the lock to avoid deadlock in async block

            task::spawn(async move {
                // Your asynchronous code her
                if let Ok(current_dir) = env::current_dir() {
                    println!("Current directory: {:?}", current_dir);
                } else {
                    eprintln!("Failed to get current directory");
                }

                let current_time = time_util::get_current_hour();
                let mp3_path = time_util::format_mp3_path(current_time);
                println!("MP3 Path: {}", mp3_path);

                // Adjust volume based on button press  
                println!("Volume: {}", volume);
                play_mp3_file(&mp3_path, volume).await;

            });
        }
    });

   // Spawn the report_punctually task
   let report_task = tokio::spawn(report_punctually(3)); // Report every 60 seconds

   // Wait for the report_task to finish (it never does in this case)
  // let _ = report_task.await;


    wind.end();
    wind.make_resizable(true);
    wind.show();

    app.run().unwrap();
}

async fn report_punctually(interval_seconds: u64) {
    let mut ticker = interval(Duration::from_secs(interval_seconds));

    loop {
        ticker.tick().await;
        println!("Report: Something happened!");
        // Add your reporting logic here
    }
}

async fn play_mp3_file(file_path: &str, volume: f32) {
    // Open the file
    let file = File::open(file_path).expect("Failed to open file");

    // Create a buffered reader
    let reader = BufReader::new(file);

    // Decode the MP3 file
    let source = Decoder::new(reader).expect("Failed to decode MP3");

    // Create a sink
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    // Add the source to the sink
    sink.append(source);

    sink.set_volume(volume);

    // Sleep for the duration of the audio
    thread::sleep(Duration::from_secs(15)); // Adjust this according to the length of your audio
}
