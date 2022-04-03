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
    }
}

table! {
    products_limit (id) {
        id -> Int4,
        products_id -> Int4,
        build_limit -> Nullable<Varchar>,
        bandwith_limit -> Nullable<Varchar>,
        site_limit -> Nullable<Varchar>,
    }
}

joinable!(products -> hosts (hosts_id));
joinable!(products_limit -> products (products_id));

allow_tables_to_appear_in_same_query!(
    hosts,
    products,
    products_limit,
);
