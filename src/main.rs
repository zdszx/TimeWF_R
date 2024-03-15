use fltk::{
    app, enums::Color, enums::Font, frame::Frame, image::SharedImage, prelude::*, text::TextBuffer,
    text::TextDisplay, window::Window,
};
use fltk::{app::*, button::*, enums::*, image::*, prelude::*, window::*};
use std::error::Error;

use async_std::task;
use rodio::{Decoder, Sink};
use std::env;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::{fs::File, io::BufReader};
use tokio::time::interval;

mod time_util;

#[tokio::main]
async fn main() {
    let app = App::default().with_scheme(app::Scheme::Gleam);
    let mut wind = Window::new(100, 100, 400, 800, "Async FLTK Example");

    //背景图加载
    load_image_into_frame("./pic/setpainting_0.png");
    display_words("Hello, world!");

    //音量按钮
    let mut button = Button::new(300, 700, 80, 50, "音量");
    button.set_color(Color::from_rgb(0, 180, 255));
    button.set_label_color(Color::White);

    let volume: Arc<Mutex<f32>> = Arc::new(Mutex::new(0.5));
    button.set_callback({
        let volume = Arc::clone(&volume);
        move |_| {
            let mut volume = volume.lock().unwrap();
            *volume += 0.1;
            if *volume > 4.0 {
                *volume = 1.0;
            }
            let volume1: f32 = *volume;
            drop(volume);
            println!("Volume1: {}", volume1);

            task::spawn(async move {
                // Your asynchronous code her
                if let Ok(current_dir) = env::current_dir() {
                    println!("Current directory: {:?}", current_dir);
                } else {
                    eprintln!("Failed to get current directory");
                }

                let current_time = time_util::get_current_hour();
                let mp3_path = time_util::format_mp3_path(current_time);
                // Adjust volume based on button press
                println!("Volume: {}", volume1);
                play_mp3_file(&mp3_path, volume1).await;
            });
        }
    });

    //定时报时后台线程
    // let volume1: f32 = 1.0; // Clone volume for async block
    let volume_clone = Arc::clone(&volume);
    let volume1 = *volume_clone.lock().unwrap();
    let report_task: tokio::task::JoinHandle<()> = tokio::spawn(report_punctually(3600, volume1));

    wind.end();
    wind.make_resizable(true);
    wind.show();
    app.run().unwrap();
}

fn display_words(words: &str) {
    let mut buffer = TextBuffer::default();
    buffer.set_text(words);

    let mut text_display = TextDisplay::new(300, 50, 80, 80, "");
    text_display.set_buffer(Some(buffer));

    text_display.set_text_color(Color::Red);
    text_display.set_text_font(Font::HelveticaBold);
    text_display.set_text_size(12);
    text_display.set_frame(FrameType::NoBox);
}

async fn report_punctually(interval_seconds: u64, volume: f32) {
    let mut ticker: tokio::time::Interval = interval(Duration::from_secs(interval_seconds));

    loop {
        ticker.tick().await;
        println!("Report: Something happened!");
        // Add your reporting logic here
        if time_util::is_on_the_hour() {
            println!("Report: 1");

            let current_time = time_util::get_current_hour();
            let mp3_path = time_util::format_mp3_path(current_time);
            play_mp3_file(&mp3_path, volume).await;
        }
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

fn load_image_into_frame(path: &str) {
    let mut frame = Frame::default_fill();
    let image_result = SharedImage::load(path);
    match image_result {
        Ok(image) => {
            frame.set_image(Some(image));
        }
        Err(e) => {
            eprintln!("Failed to load image: {}", e);
        }
    }
}
