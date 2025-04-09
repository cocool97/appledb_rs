# Installation

## Server & CLI

> Before these actions can be made, you have to build the static website by running `build_web.sh` script

### Via rust tools

```bash
# Start server
cargo run --bin appledb_server -- --config config.yaml

# Start CLI
cargo run --bin appledb_cli -- help
```

### Via debian package installation

Both CLI and server packages can build debian packages using useful `cargo-deb` tool.

```bash
cargo-deb -p appledb_server
cargo-deb -p appledb_cli
```

These packages can then be installed on a server.

An example systemd service file named `appledb_server.service` is provided and can be use to declare the server as a service.

### Via docker image

The given `Dockerfile` has multiple stages and builds the server, the static website before packaging them inside an `alpine` image

```bash
# Build
podman/docker build -t appledb -f Dockerfile .

# Run
podman/docker run \
--rm -it \
--name appledb \
-v $(pwd)/config.yaml:/app/config.yaml \
-v $(pwd)/data:/data \
-p 4000:4000 \
appledb
```

## System dependencies

The server can use either `SQLite` or `PostgreSQL` as database backend.

- SQLite setup is automatically done by the server, depending on location chosen in the configuration file
- If choosing PostgreSQL, installation process has to be done by the user externally