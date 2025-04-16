# Development tips

## Re-generate database models

The server can automatically generate models from a live database. This is useful to have models that actually match database types.

They can be generated using following command (for a local SQLite database):

```bash
sea-orm-cli generate entity --output-dir entity/src/entities/ --database-url sqlite://appledb.sqlite
```

> When using SQLite as a database for models generation, `INTEGER PRIMARY KEY` is mapped to an `i32` type. We need i64 into our code to prevent overflows. A simple command can be applied to change these models:

```bash
find entity/src/entities/ -type f -name "*.rs" -exec sed -i 's/pub id: i32/pub id: i64/' {} +
```