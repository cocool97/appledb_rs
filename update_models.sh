#!/usr/bin/env bash

# Script downloading the latest Apple devices and outputting an appledb-compatible JSON

DEVICES_URL="https://gist.githubusercontent.com/adamawolf/3048717/raw/Apple_mobile_device_types.txt"

REMOVE_KEYS="i386 x86_64 arm64"

curl -s "$DEVICES_URL" \
| jq -Rn --arg remove_keys "$REMOVE_KEYS" '
  reduce inputs as $line ({};
    if ($line | test(":")) then
      . + {
        ($line | split(":")[0] | gsub("^\\s+|\\s+$"; "")):
        ($line | split(":")[1] | gsub("^\\s+|\\s+$"; ""))
      }
    else
      .
    end
  )
  | ( $remove_keys | split(" ") ) as $keys
  | reduce $keys[] as $k (.;
      del(.[$k])
    )
'
