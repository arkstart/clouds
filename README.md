# Endpoints
* Note: `*` means required
## A. Host
1. GET /api/host -> Get All Host
2. GET /api/host/{host_name} -> Get One Host
* `{host_name}` Should be string
3. POST /api/host -> Add New Host
* Body
```
    name: String,  *
    description: String,  *
    url: String,  *
```

## B. Product
1. GET /api/product -> Get All Product
2. GET /api/product/{product_name} -> Get One Product
* `{product_name}` Should be string
2. POST /api/product -> Add New Product
* Body
```
    host_name: String,*
    name: String,*
    description: String,*
    url: String,*
    build_limit: String,
    bandwith_limit: String,
    site_limit: String,
    https_support: Bool,
    free_domain: Bool,
    custom_domain: Bool,
    domain_extension: String
```
