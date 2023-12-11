use chrono::{NaiveDateTime, Utc};
use colored::Colorize;
use std::io;

const DATEINPUT_FORMAT: &str = "%Y-%m-%d %H:%M";

#[derive(Debug)]
pub struct UserInput {
    pub title: String,
    pub date: NaiveDateTime,
    pub link: Option<String>,
}

pub fn get_user_inputs() -> UserInput {
    UserInput {
        title: get_title(),
        date: get_date(),
        link: get_optional_link(),
    }
}

fn get_stdin(mut out: String) -> String {
    io::stdin()
        .read_line(&mut out)
        .expect("Failed to read line");

    return out.trim_end().to_string();
}

fn get_title() -> String {
    println!("{}{}", "◆".green(), " Tournament Title:".white());

    let title = get_stdin(String::new());

    if title.is_empty() {
        println!("{}", "No Title, Try Again!".red().bold());
        return get_title();
    }

    return title;
}

fn get_date() -> NaiveDateTime {
    println!(
        "{}{}{}",
        "◆".green(),
        " Tournament Date (YYYY-MM-DD HH:MM):".white(),
        "\n(Leave Empty For Current Date & Time)".bright_black()
    );

    let date_string = get_stdin(String::new());

    // Return current if empty
    if date_string.is_empty() {
        return Utc::now().naive_local();
    }

    let naive_date = NaiveDateTime::parse_from_str(&date_string, DATEINPUT_FORMAT);

    let date = match naive_date {
        Ok(date) => date,
        Err(error) => {
            println!(
                "{}{:?}{}",
                "\nFailed to parse date: ".red().bold(),
                error.kind(),
                ", Try again".red().bold()
            );
            get_date()
        }
    };

    return date;
}

fn get_optional_link() -> Option<String> {
    println!(
        "{}{}",
        "◆".green(),
        " Tournament Media Link (Optional):".white()
    );

    let link = get_stdin(String::new());

    let result: Option<String> = if link.is_empty() { None } else { Some(link) };

    result
}
