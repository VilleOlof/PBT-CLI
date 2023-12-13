// @generated automatically by Diesel CLI.

diesel::table! {
    #[allow(non_snake_case)]
    #[sql_name = "Match"]
    Match_ (id) {
        #[max_length = 191]
        id -> Varchar,
        #[max_length = 191]
        matchType -> Varchar,
        matchIndex -> Integer,
        tournamentId -> Nullable<Integer>,
    }
}

diesel::table! {
    #[allow(non_snake_case)]
    MatchUser (id) {
        #[max_length = 191]
        id -> Varchar,
        #[max_length = 191]
        username -> Varchar,
        #[max_length = 191]
        userId -> Varchar,
        rank -> Integer,
        #[max_length = 191]
        lifeStatus -> Varchar,
        #[max_length = 191]
        immuneStatus -> Varchar,
        #[max_length = 191]
        matchId -> Nullable<Varchar>,
    }
}

diesel::table! {
    #[allow(non_snake_case)]
    NextTournament (id) {
        id -> Integer,
        date -> Datetime,
        #[max_length = 191]
        title -> Varchar,
        #[max_length = 191]
        description -> Varchar,
        #[max_length = 191]
        link -> Nullable<Varchar>,
    }
}

diesel::table! {
    #[allow(non_snake_case)]
    Tournament (id) {
        id -> Integer,
        version -> Integer,
        date -> Datetime,
        #[max_length = 191]
        title -> Varchar,
        #[max_length = 191]
        link -> Nullable<Varchar>,
    }
}

diesel::table! {
    #[allow(non_snake_case)]
    User (userId) {
        #[max_length = 191]
        userId -> Varchar,
        #[max_length = 191]
        username -> Varchar,
        wins -> Integer,
        ranking -> Integer,
    }
}

diesel::joinable!(Match_ -> Tournament (tournamentId));
diesel::joinable!(MatchUser -> Match_ (matchId));
diesel::joinable!(MatchUser -> User (userId));

diesel::allow_tables_to_appear_in_same_query!(Match_, MatchUser, NextTournament, Tournament, User,);
