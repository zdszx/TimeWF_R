use async_std::task;
use encoding_rs::*;
use fltk::{
    app, enums::Color, enums::Font, enums::*, frame::Frame, image::SharedImage, prelude::*,
    text::TextBuffer, text::TextDisplay, window::Window,
};
use fltk::{app::*, button::*, enums::*, image::*, prelude::*, window::*};
use fltk::{prelude::*, *};
use rodio::{Decoder, Sink};
use std::env;
use std::error::Error;
use std::io::Read;
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

    //台词框
    let mut buffer = TextBuffer::default();
    let mut text_display = TextDisplay::new(320, 20, 90, 100, "");
    text_display.set_buffer(Some(buffer.clone()));
    text_display.set_text_color(Color::Red);
    text_display.set_text_font(Font::HelveticaBold);
    text_display.set_text_size(12);
    text_display.set_frame(FrameType::NoBox);
    text_display.wrap_mode(fltk::text::WrapMode::AtBounds, 100);

    //背景图加载
    load_image_into_frame("./pic/setpainting_0.png");

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

            task::spawn({
                let mut buffer = buffer.clone();
                let mut text_display = text_display.clone();
                async move {
                    // Your asynchronous code her
                    if let Ok(current_dir) = env::current_dir() {
                        println!("Current directory: {:?}", current_dir);
                    } else {
                        eprintln!("Failed to get current directory");
                    }

                    let current_time = time_util::get_current_hour();
                    display_words(current_time, &mut buffer, &mut text_display);
                    let mp3_path = time_util::format_mp3_path(current_time);
                    play_mp3_file(&mp3_path, volume1).await;
                    // Clear the text
                    buffer.set_text("");
                }
            });
        }
    });

    //定时报时后台线程
    let volume_clone = Arc::clone(&volume);
    let volume1 = *volume_clone.lock().unwrap();
    let report_task: tokio::task::JoinHandle<()> = tokio::spawn(report_punctually(3600, volume1));

    wind.end();
    wind.make_resizable(true);
    wind.show();
    app.run().unwrap();
}

fn display_words(hour: u32, buffer: &mut TextBuffer, text_display: &mut TextDisplay) {
    let last_string = format!("{:02}", hour);
    let formatted_path = format!("./words/0/{}.txt", last_string);

    //GBK to utf-8
    let mut file = match File::open(&formatted_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to open file: {}", e);
            return;
        }
    };

    let mut bytes = Vec::new();
    if let Err(e) = file.read_to_end(&mut bytes) {
        eprintln!("Failed to read file: {}", e);
        return;
    }

    let (cow, _encoding_used, _had_errors) = GBK.decode(&bytes);

    buffer.set_text(&cow);
    text_display.set_buffer(Some(buffer.clone()));
}

async fn report_punctually(interval_seconds: u64, volume: f32) {
    let mut ticker: tokio::time::Interval = interval(Duration::from_secs(interval_seconds));

    loop {
        ticker.tick().await;
        println!("Report: Something happened!");
        // Add your reporting logic here
        if time_util::is_on_the_hour() {
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
