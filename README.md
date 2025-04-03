<p align="center" style="text-align: center">
  <img src="assets/logo.png" width="33%">
</p>

<p align="center">
    <p align="center">Apple artifacts database</p>
    <p align="center">
        <!-- <a href="https://crates.io/crates/appledb_rs">
            <img alt="crates.io" src="https://img.shields.io/crates/v/appledb_rs.svg"/>
        </a>
        <a href="https://github.com/cocool97/appledb_rs/actions">
            <img alt="ci status" src="https://github.com/cocool97/appledb_rs/actions/workflows/rust-build.yml/badge.svg"/>
        </a>
        <a href="https://deps.rs/repo/github/cocool97/appledb_rs">
            <img alt="dependency status" src="https://deps.rs/repo/github/cocool97/appledb_rs/status.svg"/>
        </a>
        <a href="https://opensource.org/licenses/MIT">
            <img alt="dependency status" src="https://img.shields.io/badge/License-MIT-yellow.svg"/>
        </a> -->
    </p>
</p>

Database storing various information about Apple internals on a per platform/version basis.

Currently stored:

- Entitlements storage (list, diff, dump...)

Main features:

- Full access via API calls, CLI or WebUI
- Designed to be easily extensible
- Full offline !
- [**TODO**] Private headers database

## Quickstart

```bash
# Start server
RUST_LOG=info cargo run --bin appledb_server -- --config config.yaml

# Start CLI
cargo run --bin appledb_cli -- help
```

## API documentation

A swagger documentation is available at path `/api/v1/swagger/#/`

## Useful commands

### Mount local IPSW file

```bash
# 1st method: Manually
## Download AEA fcs-keys
ipsw extract --fcs-key IPSW_FILE
## Extract IPSW filesystem
ipsw fw aea --pem EXTRACTED_AEA_PEM_FILE EXTRACTED_AEA_FILE
## Mount it
apfs-fuse IMG_FILE MOUNT_POINT

# 2nd method: All-in-one
ipsw mount fs IPSW_FILE
```

### Add entitlements from an IPSW

TODO

Some features may still be missing, all pull requests are welcome !
