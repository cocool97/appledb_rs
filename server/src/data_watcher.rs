use crate::{
    handlers::{post_executable_entitlements_public, post_executable_frameworks_public},
    models::AppState,
};
use anyhow::{Context, Result, anyhow, bail};
use appledb_common::api_models::TaskSource;
use futures::stream::StreamExt;
use inotify::{Event, Inotify, WatchMask};
use std::{
    ffi::OsString,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::fs::File;

pub struct DataWatcher {
    inotify: Inotify,
    state: Arc<AppState>,
    watch_root_path: PathBuf,
}

impl DataWatcher {
    pub fn new(watch_root_path: PathBuf, state: Arc<AppState>) -> Result<Self> {
        let inotify = Inotify::init()?;

        inotify
            .watches()
            .add(
                &watch_root_path,
                WatchMask::CLOSE_WRITE | WatchMask::MOVED_TO,
            )
            .context("cannot add watcher")?;

        Ok(Self {
            inotify,
            state,
            watch_root_path,
        })
    }

    pub async fn start_watch(self) -> Result<()> {
        let mut buffer = [0; 1024];
        let mut stream = self.inotify.into_event_stream(&mut buffer)?;

        while let Some(Ok(event)) = stream.next().await {
            let state = self.state.clone();
            let watch_root_path = self.watch_root_path.clone();

            tokio::spawn(async move {
                if let Err(e) = Self::handle_event(&watch_root_path, state, event).await {
                    log::error!("{e}")
                }
            });
        }

        Ok(())
    }

    pub async fn handle_event(
        watch_root_path: &Path,
        state: Arc<AppState>,
        event: Event<OsString>,
    ) -> Result<()> {
        log::debug!("got event {:?}", event.mask);
        let file_path = event
            .name
            .as_ref()
            .map(PathBuf::from)
            .ok_or(anyhow!("no file path provided in event"))?;

        let full_file_path = watch_root_path.join(file_path);

        let extension = full_file_path.extension().map(|e| e.as_encoded_bytes());

        match extension {
            Some(b"entitlements") | Some(b"ent") => {
                log::info!("got entitlements at path {}", full_file_path.display());
                let f = File::open(full_file_path).await?;
                let entitlements = serde_json::from_reader(&mut f.into_std().await)
                    .with_context(|| "cannot parse entitlements content")?;
                post_executable_entitlements_public(state, entitlements, TaskSource::Local).await?;
            }
            Some(b"framework") | Some(b"frameworks") => {
                log::info!("got frameworks at path {}", full_file_path.display());
                let f = File::open(full_file_path).await?;
                let frameworks = serde_json::from_reader(&mut f.into_std().await)
                    .with_context(|| "cannot parse frameworks content")?;
                post_executable_frameworks_public(state, frameworks, TaskSource::Local).await?;
            }
            _ => {
                bail!("unknown extension for path {}", full_file_path.display())
            }
        }

        Ok(())
    }
}
