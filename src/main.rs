use std::{
    env, thread,
    time::{self, Instant},
};

use crate::input::UserInput;

use colored::{ColoredString, Colorize};

mod db;
mod input;
mod tournament_parser;

mod models;
mod schema;

const CMD_PROCESSING_STEPS: i32 = 6;

fn cmd_step(msg: ColoredString, step: i32) {
    println!(
        "{}{}{}{}",
        "[".bright_black(),
        format!("{step}/{CMD_PROCESSING_STEPS}").white(),
        "] ".bright_black(),
        msg
    );
}

fn wait(sec: f32) {
    let duration_sec = time::Duration::from_secs_f32(sec);
    thread::sleep(duration_sec);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let tournament_file = args.get(1).expect("No Tournament File");
    let run_type: &str = match cfg!(debug_assertions) {
        true => "Debug",
        false => "Release",
    };

    println!(
        "{}{}",
        "<- Party Bots Tournament Upload CLI ->\n".blue().bold(),
        format!("Written In Rust, By @Olof | {}\n", run_type)
            .bright_black()
            .italic()
    );

    let user_input: UserInput = input::get_user_inputs();

    let elapsed_start = Instant::now();

    cmd_step("Parsing Tournament File...".bright_black().italic(), 1);
    let parsed_tournament = tournament_parser::parse(tournament_file, &user_input);
    cmd_step("Finished Parsing File".green(), 2);

    cmd_step("Connecting To MySql Database...".bright_black().italic(), 3);
    let (db_conn, db_name) = &mut db::establish_connection();
    cmd_step(
        format!("Established MySql Database Connection [{}]", db_name).green(),
        4,
    );

    cmd_step("Inserting Into DB...".bright_black().italic(), 5);
    db::insert_parsed_tournament(db_conn, parsed_tournament);
    cmd_step("Finished Inserting Tournament Into Database".green(), 6);

    println!(
        "{}{}",
        "\n[Finished Uploading Tournament To Server]\n"
            .green()
            .bold(),
        format!(
            "◆ Upload took: {}s\n",
            (elapsed_start.elapsed().as_millis() as f32 / 1000.0)
        )
        .bright_black()
        .italic()
    );
    println!("{}", "Terminal will close in 10s".bright_black().italic());

    // Wait 10 seconds then exit automatically
    wait(10.0);
}
