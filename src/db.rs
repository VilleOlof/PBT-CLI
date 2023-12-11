use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::{env, vec};
use uuid::Uuid;

use crate::models::{
    self, Match, NewMatch, NewMatchUser, NewTournament, NewUser, Tournament, User,
};
use crate::schema::match_::{matchIndex, tournamentId};
use crate::tournament_parser::{MatchUser, ParsedTournament};

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("No DATABASE_URL In .env");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error Connecting To {}", database_url))
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

    create_matches(conn, matches);
    create_users(conn, users);

    // TODO
    // Implement GetOverallPlayerList (Reference: Tournament.ts)
    // Then Implement The User Ranking Update Query With `Overall.len() - i`;
    // And then do queries for the ` _tournamenttouser` table, which is just tournament_id & userIds
}

fn create_tournament(conn: &mut MysqlConnection, new: NewTournament) -> Tournament {
    use crate::schema::tournament;

    conn.transaction(|conn| {
        diesel::insert_into(tournament::table)
            .values(&new)
            .execute(conn)?;

        tournament::table
            .order(tournament::id.desc())
            .select(Tournament::as_select())
            .first(conn)
    })
    .expect("Error while saving post")
}

fn create_matches(conn: &mut MysqlConnection, matches: Vec<(NewMatch, Vec<NewMatchUser>)>) {
    use crate::schema::match_;
    use crate::schema::matchuser;

    for (new_match, match_users) in matches {
        diesel::insert_into(match_::table)
            .values(&new_match)
            .execute(conn)
            .expect("Failed to insert new match");

        diesel::insert_into(matchuser::table)
            .values(&match_users)
            .execute(conn)
            .expect("Failed to insert new match users");
    }
}

fn create_users(conn: &mut MysqlConnection, users: Vec<NewUser>) {
    use crate::schema::user;

    for new_user in users {
        diesel::insert_into(user::table)
            .values(&new_user)
            .on_conflict_do_nothing()
            .execute(conn)
            .expect("Failed to insert new user");

        diesel::update(user::table)
            .filter(user::userId.eq(&new_user.userId))
            .set(user::username.eq(new_user.username))
            .execute(conn)
            .expect("Failed to update user: username");

        // diesel::update(user::table)
        //     .filter(user::userId.eq(&new_user.userId))
        //     .set(user::ranking.eq(user::ranking + ))
        //     .execute(conn)
        //     .expect("Failed to update user: username");
    }
}
