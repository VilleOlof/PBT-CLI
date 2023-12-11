use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    str::FromStr,
};

use chrono::{NaiveDateTime, Utc};

use crate::input::UserInput;

#[derive(Debug)]
pub struct ParsedTournament {
    pub version: i32,
    pub matches: Vec<Match>,
    pub date: NaiveDateTime,
    pub title: String,
    pub link: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Match {
    pub match_type: MatchType,
    pub players: Vec<MatchUser>,
    pub match_index: i32,
}

#[derive(Debug, Clone)]
pub struct MatchUser {
    pub username: String,
    pub user_id: String,
    pub rank: i32,
    pub life_status: LifeStatus,
    pub immune_status: ImmuneStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MatchType {
    Final,
    SemiFinal,
    QuarterFinal,
    Bonus,
    Game4,
    Game3,
    Game2,
    Game1,
}
#[derive(Debug, Clone, PartialEq)]
pub enum LifeStatus {
    Alive,
    Eliminated,
    Playing,
}
#[derive(Debug, Clone)]
pub enum ImmuneStatus {
    Immune,
    Saved,
    None,
}

impl MatchType {
    pub fn to_str(&self) -> &'static str {
        match self {
            MatchType::Final => "final",
            MatchType::SemiFinal => "semifinal",
            MatchType::QuarterFinal => "quarterfinal",
            MatchType::Bonus => "bonus",
            MatchType::Game4 => "game_4",
            MatchType::Game3 => "game_3",
            MatchType::Game2 => "game_2",
            MatchType::Game1 => "game_1",
        }
    }
}
impl FromStr for MatchType {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "Final" => Ok(MatchType::Final),
            "Semifinal" => Ok(MatchType::SemiFinal),
            "Quarterfinal" => Ok(MatchType::QuarterFinal),
            "Bonus" => Ok(MatchType::Bonus),
            "Game 4" => Ok(MatchType::Game4),
            "Game 3" => Ok(MatchType::Game3),
            "Game 2" => Ok(MatchType::Game2),
            "Game 1" => Ok(MatchType::Game1),
            _ => Err(()),
        }
    }
}

impl LifeStatus {
    pub fn to_str(&self) -> &'static str {
        match self {
            LifeStatus::Alive => "alive",
            LifeStatus::Eliminated => "eliminated",
            LifeStatus::Playing => "playing",
        }
    }
}
impl FromStr for LifeStatus {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "a" => Ok(LifeStatus::Alive),
            "e" => Ok(LifeStatus::Eliminated),
            "p" => Ok(LifeStatus::Playing),
            _ => Err(()),
        }
    }
}

impl ImmuneStatus {
    pub fn to_str(&self) -> &'static str {
        match self {
            ImmuneStatus::Immune => "immune",
            ImmuneStatus::Saved => "saved",
            ImmuneStatus::None => "none",
        }
    }
}
impl FromStr for ImmuneStatus {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "i" => Ok(ImmuneStatus::Immune),
            "is" => Ok(ImmuneStatus::Saved),
            "" => Ok(ImmuneStatus::None),
            _ => Err(()),
        }
    }
}

pub fn parse(file_path: &String, user_input: &UserInput) -> ParsedTournament {
    let mut tournament = ParsedTournament {
        version: -1,
        matches: vec![],
        date: Utc::now().naive_local(),
        title: String::new(),
        link: None,
    };

    let lines = get_lines(&file_path).expect("Failed to get lines");
    tournament.version = lines[0]
        .replace("v", "")
        .parse::<i32>()
        .expect("Failed to parse version");
    tournament.date = user_input.date;
    tournament.title = user_input.title.to_owned();
    tournament.link = user_input.link.to_owned();

    for mut i in 0..lines.len() {
        let raw_line = lines[i].to_owned();
        let line = raw_line.trim();
        if raw_line.is_empty() {
            continue;
        }

        let match_type = get_match_type(line);

        if match_type.is_none() {
            continue;
        }

        let mut tournament_match: Match = Match {
            match_type: match_type.unwrap(),
            players: vec![],
            match_index: -1,
        };

        i += 1;
        let mut player_line = lines[i].trim().to_owned();
        while !player_line.is_empty() {
            let player: MatchUser = parse_player(&player_line);
            tournament_match.players.push(player);

            i += 1;
            player_line = lines[i].trim().to_owned();
        }

        tournament.matches.push(tournament_match);
    }

    // Add the indexes
    for i in 0..tournament.matches.len() {
        tournament.matches[i].match_index = i as i32;
    }

    tournament
}

pub fn get_overall_player_list(matches: Vec<Match>) -> Vec<MatchUser> {
    let mut players: Vec<MatchUser> = vec![];
    for m in matches {
        let mut new_players: Vec<MatchUser> = m
            .players
            .into_iter()
            .filter(|p| p.life_status != LifeStatus::Alive || m.match_type == MatchType::Final)
            .collect();

        players.append(&mut new_players);
    }

    players
}

fn get_lines(filename: &str) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn get_match_type(line: &str) -> Option<MatchType> {
    let match_line = &line.replace("#", "");

    let match_type = MatchType::from_str(&match_line);
    if match_type.is_err() {
        None
    } else {
        Some(match_type.unwrap())
    }
}

fn parse_player(player_line: &str) -> MatchUser {
    let parts: Vec<String> = player_line.splitn(5, ':').map(|p| p.to_owned()).collect();
    MatchUser {
        username: parts[4].to_owned(),
        user_id: parts[3].to_owned(),
        rank: parts[2]
            .parse::<i32>()
            .expect("Failed to parse player rank"),
        life_status: LifeStatus::from_str(&parts[1]).expect("Failed to Parse LifeStatus"),
        immune_status: ImmuneStatus::from_str(&parts[0]).expect("Failed to Parse ImmuneStatus"),
    }
}
