# Endpoints
* Note: `*` means required
## A. Host
1. GET /api/hosts -> Get All Host
2. GET /api/hosts/{host_name} -> Get One Host
* `{host_name}` Should be string
3. GET /api/hosts/products/{host_name} -> Get All Product of the Host
* `{host_name}` Should be string
4. POST /api/hosts -> Add New Host
* Body
```
    name: String,  *
    description: String,  *
    url: String,  *
```

## B. Product
1. GET /api/products -> Get All Product
2. GET /api/products/{product_name} -> Get One Product
* `{product_name}` Should be string
2. POST /api/products -> Add New Product
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
