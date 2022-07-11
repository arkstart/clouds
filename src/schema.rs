table! {
    hosts (id) {
        id -> Int4,
        name -> Varchar,
        description -> Varchar,
        url -> Varchar,
        always_free -> Nullable<Bool>,
        free_tier -> Nullable<Bool>,
        frontend_support -> Nullable<Bool>,
        backend_support -> Nullable<Bool>,
        database_support -> Nullable<Bool>,
        template -> Nullable<Varchar>,
    }
}

table! {
    plans (id) {
        id -> Int4,
        hosts_id -> Int4,
        name -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        price -> Nullable<Int4>,
        price_unit -> Nullable<Varchar>,
        price_timeunit -> Nullable<Varchar>,
        price_desc -> Nullable<Varchar>,
        concurrent_build -> Nullable<Int4>,
        concurrent_build_unit -> Nullable<Varchar>,
        concurrent_build_timeunit -> Nullable<Varchar>,
        concurrent_build_desc -> Nullable<Varchar>,
        bandwidth -> Nullable<Int4>,
        bandwidth_unit -> Nullable<Varchar>,
        bandwidth_timeunit -> Nullable<Varchar>,
        bandwidth_desc -> Nullable<Varchar>,
        build -> Nullable<Int4>,
        build_unit -> Nullable<Varchar>,
        build_timeunit -> Nullable<Varchar>,
        build_desc -> Nullable<Varchar>,
        analytic -> Nullable<Bool>,
        analytic_price -> Nullable<Int4>,
        analytic_unit -> Nullable<Varchar>,
        analytic_timeunit -> Nullable<Varchar>,
        analytic_desc -> Nullable<Varchar>,
        plan_url -> Nullable<Varchar>,
        currency -> Nullable<Varchar>,
        discounted_price -> Nullable<Int4>,
        free_domain -> Nullable<Bool>,
        domain_extension -> Nullable<Varchar>,
        database_benefit -> Nullable<Bool>,
        page_data -> Nullable<Varchar>,
        // supported_languages -> Nullable<Varchar>,
    }
}

table! {
    products (id) {
        id -> Int4,
        hosts_id -> Int4,
        title -> Varchar,
        subtitle -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        category -> Varchar,
        product_url -> Nullable<Varchar>,
        free_tier -> Nullable<Bool>,
        free_trial -> Nullable<Bool>,
        base_price -> Nullable<Float8>,
        price_unit -> Nullable<Varchar>,
        price_timeunit -> Nullable<Varchar>,
        price_desc -> Nullable<Varchar>,
        multi_pricing -> Nullable<Bool>,
    }
}

joinable!(plans -> hosts (hosts_id));
joinable!(products -> hosts (hosts_id));

allow_tables_to_appear_in_same_query!(
    hosts,
    plans,
    products,
);
