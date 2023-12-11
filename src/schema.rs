// @generated automatically by Diesel CLI.

diesel::table! {
    #[sql_name = "match"]
    match_ (id) {
        #[max_length = 191]
        id -> Varchar,
        #[max_length = 191]
        matchType -> Varchar,
        matchIndex -> Integer,
        tournamentId -> Nullable<Integer>,
    }
}

diesel::table! {
    matchuser (id) {
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
    nexttournament (id) {
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
    tournament (id) {
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
    user (userId) {
        #[max_length = 191]
        userId -> Varchar,
        #[max_length = 191]
        username -> Varchar,
        wins -> Integer,
        ranking -> Integer,
    }
}

diesel::joinable!(match_ -> tournament (tournamentId));
diesel::joinable!(matchuser -> match_ (matchId));
diesel::joinable!(matchuser -> user (userId));

diesel::allow_tables_to_appear_in_same_query!(match_, matchuser, nexttournament, tournament, user,);
