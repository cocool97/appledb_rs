# Server configuration

> Configuration file is in `YAML` format and all fields are mandatory to have a more explicit behavior

## Example

```yaml
listen_mode: http://127.0.0.1:4000

http_max_body_size: 10485760  # 10 MB

database_url: sqlite://appledb.sqlite

web_sources_path: web/dist

max_concurrent_tasks: 10

serve_openapi: false

cors_allowed_origins:
  - https://mydomain1
  - https://mydomain2
```

## Configuration reference

|Field|Meaning|Type|
|:---|:-----|:--:|
|listen_mode| Server listen mode URL (starting either with `http://` or `unix://`) |`string`|
|http_max_body_size| Maximum http body size allowed for HTTP requests |`integer`|
|database_url| URL describing database type & parameters (must start with *sqlite://* or *postgres://*) |`string`|
|web_sources_path| Path to static website files|`string`|
|max_concurrent_tasks| Maximum concurrent insertion tasks | `integer`|
|serve_openapi| Expose or not openapi documentation |`boolean`|
|cors_allowed_origins| List of CORS domains to allow | `option[list[string]]` |
