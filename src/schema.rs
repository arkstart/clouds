table! {
    host (id) {
        id -> Int4,
        name -> Varchar,
        description -> Varchar,
        url -> Varchar,
    }
}

table! {
    shortenurl (id) {
        id -> Int4,
        origin_url -> Varchar,
        hashed_url -> Nullable<Varchar>,
        custom_url -> Nullable<Varchar>,
        redirection_count -> Nullable<Int4>,
    }
}

allow_tables_to_appear_in_same_query!(
    host,
    shortenurl,
);
