use std::path::{Path, PathBuf};

use anyhow::anyhow;
use apple_codesign::path_is_macho;
use walkdir::{IntoIter, WalkDir};

pub struct IPSWExecutablesIterator {
    walkdir: IntoIter,
}

impl IPSWExecutablesIterator {
    pub fn new<P: AsRef<Path>>(mount_path: P) -> Self {
        Self {
            walkdir: WalkDir::new(mount_path).into_iter(),
        }
    }
}

impl Iterator for IPSWExecutablesIterator {
    type Item = anyhow::Result<PathBuf>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.walkdir.next() {
            Some(item) => match item {
                Ok(item) => {
                    if let Ok(is_executable) = path_is_macho(item.path()) {
                        if is_executable {
                            return Some(Ok(item.into_path()));
                        }
                    }

                    Some(Err(anyhow!("path is not an executable")))
                }
                Err(e) => Some(Err(e.into())),
            },
            None => None,
        }
    }
}
