use diesel::connection::SimpleConnection;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;

use dotenvy::{dotenv, from_filename};
use std::{env, vec};
use url::Url;
use uuid::Uuid;

use crate::models::{self, NewMatch, NewMatchUser, NewTournament, NewUser, Tournament};
use crate::tournament_parser::{self, MatchUser, ParsedTournament};

pub fn establish_connection() -> (MysqlConnection, String) {
    match cfg!(debug_assertions) {
        true => dotenv().ok(),
        false => from_filename(".release.env").ok(),
    };

    let database_url = env::var("DATABASE_URL").expect("No DATABASE_URL In .env");
    let database_name = Url::parse(&database_url)
        .expect("Failed to parse database url")
        .path()
        .to_owned();

    let conn = MysqlConnection::establish(&database_url).unwrap_or_else(|error| {
        panic!(
            "Error Connecting To {}, {}",
            database_url,
            error.to_string()
        )
    });

    return (conn, database_name);
}

pub fn insert_parsed_tournament(conn: &mut MysqlConnection, parsed_tournament: ParsedTournament) {
    let db_tournament = create_tournament(
        conn,
        NewTournament {
            version: parsed_tournament.version,
            date: parsed_tournament.date,
            title: parsed_tournament.title,
            link: parsed_tournament.link,
        },
    );

    let mut matches: Vec<(NewMatch, Vec<NewMatchUser>)> = vec![];
    let mut users: Vec<NewUser> = vec![];

    for m in parsed_tournament.matches.clone() {
        let new_m = NewMatch {
            id: Uuid::new_v4().to_string(),
            matchType: m.match_type.to_str().to_owned(),
            matchIndex: m.match_index,
            tournamentId: Some(db_tournament.id),
        };

        let mut match_users: Vec<NewMatchUser> = vec![];
        for user in m.players {
            let new_match_user = NewMatchUser {
                id: Uuid::new_v4().to_string(),
                username: user.username.to_owned(),
                userId: user.user_id.to_owned(),
                rank: user.rank,
                lifeStatus: user.life_status.to_str().to_owned(),
                immuneStatus: user.immune_status.to_str().to_owned(),
                matchId: new_m.id.clone(),
            };
            match_users.push(new_match_user);

            fn user_exists(new_id: &String, users: &Vec<NewUser>) -> bool {
                for user in users {
                    if &user.userId == new_id {
                        return true;
                    }
                }

                return false;
            }

            if !user_exists(&user.user_id, &users) {
                let new_user = NewUser {
                    userId: user.user_id.to_owned(),
                    username: user.username.to_owned(),
                    wins: 0,
                    ranking: 0,
                };
                users.push(new_user);
            }
        }

        matches.push((new_m, match_users));
    }

    create_users(conn, users);
    create_matches(conn, matches);

    let overall = tournament_parser::get_overall_player_list(parsed_tournament.matches.clone());
    update_user_win(
        conn,
        overall.first().expect("No first user in overall").clone(),
    );
    update_user_ranks(conn, overall.clone());

    let mut linked_tournament_users: Vec<(i32, String)> = vec![];
    // Might as well utilize the overall vec for this
    for user in overall {
        linked_tournament_users.push((db_tournament.id, user.user_id));
    }
    create_tournament_user_link(conn, linked_tournament_users);
}

fn create_tournament(conn: &mut MysqlConnection, new: NewTournament) -> Tournament {
    use crate::schema::Tournament;

    conn.transaction(|conn| {
        diesel::insert_into(Tournament::table)
            .values(&new)
            .execute(conn)?;

        Tournament::table
            .order(Tournament::id.desc())
            .select(models::Tournament::as_select())
            .first(conn)
    })
    .expect("Failed to create new tournament")
}

fn create_matches(conn: &mut MysqlConnection, matches: Vec<(NewMatch, Vec<NewMatchUser>)>) {
    use crate::schema::MatchUser;
    use crate::schema::Match_;

    for (new_match, match_users) in matches {
        diesel::insert_into(Match_::table)
            .values(&new_match)
            .execute(conn)
            .expect("Failed to insert new match");

        diesel::insert_into(MatchUser::table)
            .values(&match_users)
            .execute(conn)
            .expect("Failed to insert new match users");
    }
}

fn create_users(conn: &mut MysqlConnection, users: Vec<NewUser>) {
    use crate::schema::User;

    for new_user in users {
        diesel::insert_into(User::table)
            .values(&new_user)
            .on_conflict_do_nothing()
            .execute(conn)
            .expect("Failed to insert new user");

        diesel::update(User::table)
            .filter(User::userId.eq(&new_user.userId))
            .set(User::username.eq(new_user.username))
            .execute(conn)
            .expect("Failed to update user: username");
    }
}

fn update_user_ranks(conn: &mut MysqlConnection, users: Vec<MatchUser>) {
    use crate::schema::User;

    for i in 0..users.len() {
        let user = &users[i];
        diesel::update(User::table)
            .filter(User::userId.eq(&user.user_id))
            .set(User::ranking.eq(User::ranking + (users.len() - i) as i32))
            .execute(conn)
            .expect(&format!(
                "Failed to update user ranking: ({})",
                user.username
            ));
    }
}

fn update_user_win(conn: &mut MysqlConnection, user: MatchUser) {
    use crate::schema::User;

    diesel::update(User::table)
        .filter(User::userId.eq(user.user_id))
        .set(User::wins.eq(User::wins + 1))
        .execute(conn)
        .expect(&format!("Failed to update user wins: ({})", user.username));
}

fn create_tournament_user_link(conn: &mut MysqlConnection, linked: Vec<(i32, String)>) {
    // have to do raw sql query since diesel dont work with non-primary key tables and this table is only for internal prisma

    let mut batch_query: String = String::new();
    for (a, b) in linked {
        let query = format!(
            "INSERT INTO _TournamentToUser (A, B) VALUES ({}, '{}');",
            a, b
        );

        batch_query += &query;
    }

    conn.batch_execute(&batch_query)
        .expect("Failed to batch insert tournament to user fields");
}
