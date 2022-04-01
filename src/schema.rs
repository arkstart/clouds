table! {
    hosts (id) {
        id -> Int4,
        name -> Varchar,
        description -> Varchar,
        url -> Varchar,
    }
}

table! {
    products (id) {
        id -> Int4,
        hosts_id -> Int4,
        name -> Varchar,
        description -> Varchar,
        url -> Varchar,
        free -> Bool,
        pricing -> Varchar,
    }
}

joinable!(products -> hosts (hosts_id));

allow_tables_to_appear_in_same_query!(
    hosts,
    products,
);
