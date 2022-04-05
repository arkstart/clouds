table! {
    domain_benefits (id) {
        id -> Int4,
        products_id -> Int4,
        https_support -> Nullable<Bool>,
        free_domain -> Nullable<Bool>,
        custom_domain -> Nullable<Bool>,
        domain_extension -> Nullable<Varchar>,
    }
}

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

joinable!(domain_benefits -> products (products_id));
joinable!(products -> hosts (hosts_id));
joinable!(products_limit -> products (products_id));

allow_tables_to_appear_in_same_query!(
    domain_benefits,
    hosts,
    products,
    products_limit,
);
