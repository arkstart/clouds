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
    name: String,  *
    description: String,
    url: String,
    always_free: Bool,
    free_tier: Bool
    frontend_support: Bool,
    backend_support: Bool,
    database_support: Bool
```

5. PUT /api/hosts/ -> Update Host based on Name

- Body

```
    name: String,  *
    description: String,
    url: String,
    build_limit: String,
    bandwith_limit: String,
    site_limit: String,
    https_support: Bool,
    free_domain: Bool,
    custom_domain: Bool,
    domain_extension: String
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
  pub host_name: String,
  pub title: String,
  pub subtitle: Option<String>,
  pub description: Option<String>,
  pub category: String, "Category should be either ANLT, STOR, DTBS, CMPT, or CTNR"
  pub product_url: Option<String>,
  pub free_tier: Option<bool>,
  pub free_trial: Option<bool>,
  pub base_price: Option<f64>,
  pub price_unit: Option<String>,
  pub price_timeunit: Option<String>,
  pub price_desc: Option<String>,
  pub multi_pricing: Option<bool>,
```

5. PUT /api/products/ -> Update Plan based on Name

- Body

```
  pub title: String,
  pub subtitle: Option<String>,
  pub description: Option<String>,
  pub category: Option<String>, "Category should be either ANLT, STOR, DTBS, CMPT, or CTNR"
  pub product_url: Option<String>,
  pub free_tier: Option<bool>,
  pub free_trial: Option<bool>,
  pub base_price: Option<f64>,
  pub price_unit: Option<String>,
  pub price_timeunit: Option<String>,
  pub price_desc: Option<String>,
  pub multi_pricing: Option<bool>,
```
