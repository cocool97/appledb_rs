use std::path::Path;

use anyhow::{Result, bail};
use appledb_common::Platform;

use crate::models::OptsPlatform;

pub fn read_platform_version_from_plist<P: AsRef<Path>>(
    mount_point: P,
) -> Result<(OptsPlatform, String)> {
    for plist_path_hint in [
        "System/Library/CoreServices/SystemVersion.plist",
        "root/System/Library/CoreServices/SystemVersion.plist",
    ] {
        let path = mount_point.as_ref().join(plist_path_hint);

        if path.exists() {
            if let Ok(plist_value) = plist::from_file::<_, plist::Value>(path) {
                if let Some(plist_dict) = plist_value.as_dictionary() {
                    if let Some(product_name) =
                        plist_dict.get("ProductName").and_then(|v| v.as_string())
                    {
                        let product_name = Platform::from_product_name(product_name);
                        let product_version = plist_dict
                            .get("ProductVersion")
                            .and_then(|v| v.as_string())
                            .map(|v| v.to_string());

                        match (product_name, product_version) {
                            (None, None) | (None, Some(_)) | (Some(_), None) => continue,
                            (Some(product_name), Some(product_version)) => {
                                return Ok((product_name.into(), product_version));
                            }
                        }
                    }
                }
            }
        }
    }

    bail!("cannot automatically determine system platform and version")
}
