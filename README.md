# Setup
- Install Postgresql
- Install Rust `https://www.rust-lang.org/tools/install`
- Setup .env
- $ sudo apt update
- $ sudo apt install -y pkg-config libssl-dev libpq-dev
- $ cargo build
- $ cargo install diesel_cli --no-default-features --features postgres
- $ cargo run
- Migrate it by sending POST http://localhost:8080/api/migration
- $ pg_restore --verbose --clean --no-acl --no-owner -h localhost -U ${username} -d ${dbName} latest.dump

# Endpoints
- Note: `*` means required

## A. Host

1. GET /api/hosts/ -> Get All Host
   Query Param (Optional)

```
    always_free: Bool,
    free_tier: Bool
    frontend_support: Bool,
    backend_support: Bool,
    database_support: Bool
```

2. GET /api/hosts/{host_name} -> Get One Host

- `{host_name}` Should be string

3. GET /api/hosts/products/{host_name} -> Get All Product of the Host

- `{host_name}` Should be string

4. POST /api/hosts/ -> Add New Host

- Body

```
  name: String,
  description: Option<String>,
  url: Option<String>,
  always_free: Option<bool>,
  free_tier: Option<bool>,
  frontend_support: Option<bool>,
  backend_support: Option<bool>,
  database_support: Option<bool>,
  template: Option<String>
```

5. PUT /api/hosts/ -> Update Host based on Name

- Body

```
  name: String,
  description: Option<String>,
  url: Option<String>,
  always_free: Option<bool>,
  free_tier: Option<bool>,
  frontend_support: Option<bool>,
  backend_support: Option<bool>,
  database_support: Option<bool>,
  template: Option<String>
```

## B. Plan

1. GET /api/plans/ -> Get All Plan
2. GET /api/plans/{plan_name} -> Get One Plan

- `{plan_name}` Should be string

3. GET /api/plans/hosts/{host_name} -> Get All Plan of the Host

- `{host_name}` Should be string

4. POST /api/plans/ -> Add New Plan

- Body

```
  host_name: String,
  name: Option<String>,
  description: Option<String>,
  price: Option<i32>,
  price_unit: Option<String>,
  price_timeunit: Option<String>,
  price_desc: Option<String>,
  // Concurrent Build
  concurrent_build: Option<i32>,
  concurrent_build_unit: Option<String>,
  concurrent_build_timeunit: Option<String>,
  concurrent_build_desc: Option<String>,
  // Bandwidth
  bandwidth: Option<i32>,
  bandwidth_unit: Option<String>,
  bandwidth_timeunit: Option<String>,
  bandwidth_desc: Option<String>,
  // Build
  build: Option<i32>,
  build_unit: Option<String>,
  build_timeunit: Option<String>,
  build_desc: Option<String>,
  // Analytic
  analytic: Option<bool>,
  analytic_price: Option<i32>,
  analytic_unit: Option<String>,
  analytic_timeunit: Option<String>,
  analytic_desc: Option<String>,
  plan_url: Option<String>,
```

5. PUT /api/plans/ -> Update Plan based on Name

- Body

```
  name: Option<String>,
  description: Option<String>,
  price: Option<i32>,
  price_unit: Option<String>,
  price_timeunit: Option<String>,
  price_desc: Option<String>,
  // Concurrent Build
  concurrent_build: Option<i32>,
  concurrent_build_unit: Option<String>,
  concurrent_build_timeunit: Option<String>,
  concurrent_build_desc: Option<String>,
  // Bandwidth
  bandwidth: Option<i32>,
  bandwidth_unit: Option<String>,
  bandwidth_timeunit: Option<String>,
  bandwidth_desc: Option<String>,
  // Build
  build: Option<i32>,
  build_unit: Option<String>,
  build_timeunit: Option<String>,
  build_desc: Option<String>,
  // Analytic
  analytic: Option<bool>,
  analytic_price: Option<i32>,
  analytic_unit: Option<String>,
  analytic_timeunit: Option<String>,
  analytic_desc: Option<String>,
  plan_url: Option<String>,
```

## C. Product

1. GET /api/products/ -> Get All Product
2. GET /api/products/{product_name} -> Get One Product

- `{product_name}` Should be string

3. POST /api/products/ -> Add New Product

- Body

```
  host_name: String,
  title: String,
  subtitle: Option<String>,
  description: Option<String>,
  category: String, "Category should be either ANLT, STOR, DTBS, CMPT, or CTNR"
  product_url: Option<String>,
  free_tier: Option<bool>,
  free_trial: Option<bool>,
  base_price: Option<f64>,
  price_unit: Option<String>,
  price_timeunit: Option<String>,
  price_desc: Option<String>,
  multi_pricing: Option<bool>,
```

5. PUT /api/products/ -> Update Plan based on Name

- Body

```
  title: String,
  subtitle: Option<String>,
  description: Option<String>,
  category: Option<String>, "Category should be either ANLT, STOR, DTBS, CMPT, or CTNR"
  product_url: Option<String>,
  free_tier: Option<bool>,
  free_trial: Option<bool>,
  base_price: Option<f64>,
  price_unit: Option<String>,
  price_timeunit: Option<String>,
  price_desc: Option<String>,
  multi_pricing: Option<bool>,
```
