<p align="center" style="text-align: center">
  <img src="assets/logo.png" width="33%">
</p>

<p align="center">
    <p align="center">Apple artifacts database</p>
    <p align="center">
        <!-- <a href="https://crates.io/crates/appledb_rs">
            <img alt="crates.io" src="https://img.shields.io/crates/v/appledb_rs.svg"/>
        </a> --->
        <a href="https://github.com/cocool97/appledb_rs/actions">
            <img alt="ci status" src="https://github.com/cocool97/appledb_rs/actions/workflows/rust-build.yml/badge.svg"/>
        </a>
        <a href="https://deps.rs/repo/github/cocool97/appledb_rs">
            <img alt="dependency status" src="https://deps.rs/repo/github/cocool97/appledb_rs/status.svg"/>
        </a>
        <a href="https://opensource.org/licenses/MIT">
            <img alt="dependency status" src="https://img.shields.io/badge/License-MIT-yellow.svg"/>
        </a>
    </p>
</p>

Server storing various information about Apple internals on a per platform/version basis.

Currently stored:

- Entitlements
- Executables

Main features:

- Full access via API calls, CLI or WebUI
- Designed to be easily extensible
- Works in full offline environments !

## Installation

Improved documentation available [here](./documentation/installation.md).

## Server configuration

Improved documentation available [here](./documentation/configuration.md).

## API documentation

Swagger documentations are available when running the server for both public and admin routes:

- Public routes path: `/api/v1/swagger/`
- Admin routes path: `/api/admin/swagger/`

## Useful tips

Improved documentation available [here](./documentation/tips.md).

## Missing features

- Generate and store public / private headers
- Store information about frameworks

Some features may still be missing, all pull requests are welcome !
