# Endpoints
* Note: `*` means required
## Host
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

## Product
1. GET /api/host -> Get All Host
2. POST /api/host -> Add New Host
* Body
```
    host_name: String,*
    name: String,*
    description: String,*
    url: String,*
    free: Boolean,
    pricing: String,
```
