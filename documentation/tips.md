# Some useful tips

## Mount local IPSW file

1. Manually

```bash
## Download AEA fcs-keys
ipsw extract --fcs-key IPSW_FILE
## Extract IPSW filesystem
ipsw fw aea --pem EXTRACTED_AEA_PEM_FILE EXTRACTED_AEA_FILE
## Mount it
apfs-fuse IMG_FILE MOUNT_POINT
```

2. Automatically

```bash
ipsw mount fs IPSW_FILE
```

## Add entitlements from an IPSW

With an IPSW mounted on local filesystem, a `POST` request can be made on `/api/admin/executable/entitlements` endpoint with given JSON data to push entitlements for given hardware, platform and version :

```json
{
    "platform": "ios",
    "model_code": "iPhone17,4",
    "version": "18.3.1",
    "executable_entitlements": {
        "EXECUTABLE_FULL_PATH": {
            "entitlement#1": {...},
            ...
        }
    },
}
```

## Watch a local directory for parsing results

It is possible to let `appledb_rs` watch for files dropped in a local directory.

This can be pretty useful when parsing results cannot be sent over http but are uploaded as files.

To activate local directory watch just add `watched_directory` in the configuration file, having it point to an existing directory.

Parsing is done using distinct file extensions:

- `entitlements` or `ent` for entitlement results
- `framework` or `frameworks` for framework results

Tasks run from a local upload are referred as `local` in tasks progress API results.
