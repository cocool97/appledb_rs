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