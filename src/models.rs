use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::Tournament)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Tournament {
    pub id: i32,
    pub version: i32,
    pub date: NaiveDateTime,
    pub title: String,
    pub link: Option<String>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::schema::Tournament)]
pub struct NewTournament {
    pub version: i32,
    pub date: NaiveDateTime,
    pub title: String,
    pub link: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::Match_)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Match {
    pub id: String,
    pub matchType: String,
    pub matchIndex: i32,
    pub tournamentId: Option<i32>,
}

#[allow(non_snake_case)]
#[derive(Insertable, Debug)]
#[diesel(table_name = crate::schema::Match_)]
pub struct NewMatch {
    pub id: String,
    pub matchType: String,
    pub matchIndex: i32,
    pub tournamentId: Option<i32>,
}

#[allow(non_snake_case)]
#[derive(Insertable, Debug)]
#[diesel(table_name = crate::schema::MatchUser)]
pub struct NewMatchUser {
    pub id: String,
    pub username: String,
    pub userId: String,
    pub rank: i32,
    pub lifeStatus: String,
    pub immuneStatus: String,
    pub matchId: String,
}

#[allow(non_snake_case)]
#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::User)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct User {
    pub userId: String,
    pub username: String,
    pub wins: i32,
    pub ranking: i32,
}

#[allow(non_snake_case)]
#[derive(Insertable, Debug)]
#[diesel(table_name = crate::schema::User)]
pub struct NewUser {
    pub userId: String,
    pub username: String,
    pub wins: i32,
    pub ranking: i32,
}
