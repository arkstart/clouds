table! {
    hosts (id) {
        id -> Int4,
        name -> Varchar,
        always_free -> Nullable<Bool>,
        free_tier -> Nullable<Bool>,
        frontend_support -> Nullable<Bool>,
        backend_support -> Nullable<Bool>,
        database_support -> Nullable<Bool>,
        description -> Nullable<Varchar>,
        url -> Nullable<Varchar>,
        product_based -> Nullable<Bool>,
        plan_based -> Nullable<Bool>,
    }
}

table! {
    products (id) {
        id -> Int4,
        hosts_id -> Int4,
        name -> Varchar,
        description -> Varchar,
        url -> Varchar,
        build_limit -> Nullable<Varchar>,
        bandwidth_limit -> Nullable<Varchar>,
        site_limit -> Nullable<Varchar>,
        https_support -> Nullable<Bool>,
        free_domain -> Nullable<Bool>,
        custom_domain -> Nullable<Bool>,
        domain_extension -> Nullable<Varchar>,
    }
}

joinable!(products -> hosts (hosts_id));

allow_tables_to_appear_in_same_query!(
    hosts,
    products,
);
