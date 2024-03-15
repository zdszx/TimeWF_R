
use chrono::{Local, Datelike, Timelike};

pub fn get_current_time() -> String {
    let local_time = Local::now();
    
    let hour = local_time.hour();
    let minute = local_time.minute();
    
    // Format the hour and minute into a string in 24-hour format
    let hour_str = format!("{:02}{:02}", hour, minute);
    
    // Handle the case where the time is exactly midnight (00:00)
    if hour == 0 && minute == 0 {
        return "2400".to_string();
    }
    
    hour_str
}
pub fn is_on_the_hour() -> bool {
    let now = Local::now();
    now.minute() == 0 && now.second() == 0
}

pub fn get_current_hour() -> u32 {
    let local_time = Local::now();
    local_time.hour()
}

pub fn format_mp3_path(hour: u32) -> String {
    if  hour > 23 {
        panic!("Invalid hour input");
    }
    
    let last_string = format!("{:02}", hour);
    let formatted_path = format!("./voice/0/{}00.mp3", last_string);
    
    formatted_path
}
