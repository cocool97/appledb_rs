use crate::{
    handlers::{post_executable_entitlements_public, post_executable_frameworks_public},
    models::AppState,
};
use anyhow::{Context, Result, anyhow, bail};
use appledb_common::{IPSWEntitlements, IPSWFrameworks, api_models::TaskSource};
use futures::stream::StreamExt;
use inotify::{Event, Inotify, WatchMask};
use std::{
    ffi::{OsStr, OsString},
    ops::Deref,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::{
    fs::File,
    sync::{OwnedSemaphorePermit, Semaphore},
};

pub struct DataWatcher {
    inotify: Inotify,
    state: Arc<AppState>,
    watch_root_path: PathBuf,
    semaphore: Arc<Semaphore>,
}

impl DataWatcher {
    pub fn new<P: AsRef<Path>>(watch_root_path: P, state: Arc<AppState>) -> Result<Self> {
        let inotify = Inotify::init()?;

        inotify
            .watches()
            .add(
                &watch_root_path,
                WatchMask::CLOSE_WRITE | WatchMask::MOVED_TO,
            )
            .context("cannot add watcher")?;

        let semaphore = Arc::new(Semaphore::new(state.max_concurrent_tasks));

        Ok(Self {
            inotify,
            state,
            watch_root_path: watch_root_path.as_ref().to_path_buf(),
            semaphore,
        })
    }

    pub async fn start_watch(self) -> Result<()> {
        let mut buffer = [0; 1024];
        let mut stream = self.inotify.into_event_stream(&mut buffer)?;

        while let Some(Ok(event)) = stream.next().await {
            let state = self.state.clone();
            let watch_root_path = self.watch_root_path.clone();

            let permit = self.semaphore.clone().acquire_owned().await?;
            log::debug!(
                "semaphore acquired, {} left before blocking",
                permit.num_permits()
            );

            tokio::spawn(async move {
                if let Err(e) = Self::handle_event(&watch_root_path, state, event, permit).await {
                    log::error!("got error while handling event: {e}");
                }
            });
        }

        Ok(())
    }

    async fn handle_event(
        watch_root_path: &Path,
        state: Arc<AppState>,
        event: Event<OsString>,
        permit: OwnedSemaphorePermit,
    ) -> Result<()> {
        log::debug!("got event {:?}", event.mask);
        let file_path = event
            .name
            .as_ref()
            .map(PathBuf::from)
            .ok_or(anyhow!("no file path provided in event"))?;

        let full_file_path = EventFilePath(watch_root_path.join(&file_path));

        let Some(extension) = full_file_path.extension().map(OsStr::as_encoded_bytes) else {
            bail!(
                "no extension provided for path {}",
                full_file_path.display()
            )
        };

        let mut event_file = File::open(&*full_file_path).await?.into_std().await;
        match extension {
            b"entitlements" | b"ent" => {
                log::info!("got entitlements at path {}", full_file_path.display());
                let entitlements: IPSWEntitlements = serde_json::from_reader(&mut event_file)
                    .with_context(|| "cannot parse entitlements content")?;
                post_executable_entitlements_public(state, entitlements, TaskSource::Local(permit))
                    .await?;
            }
            b"framework" | b"frameworks" => {
                log::info!("got frameworks at path {}", full_file_path.display());
                let frameworks: IPSWFrameworks = serde_json::from_reader(&mut event_file)
                    .with_context(|| "cannot parse frameworks content")?;
                post_executable_frameworks_public(state, frameworks, TaskSource::Local(permit))
                    .await?;
            }
            _ => {
                bail!("unknown extension for path {}", full_file_path.display());
            }
        }

        Ok(())
    }
}

/// Newtype for `Pathbuf` with custom `Drop` implementation
struct EventFilePath(pub PathBuf);

impl Deref for EventFilePath {
    type Target = PathBuf;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Drop for EventFilePath {
    fn drop(&mut self) {
        match std::fs::remove_file(&self.0) {
            Ok(()) => log::info!("removed event file at path {}", self.0.display()),
            Err(e) => log::error!("cannot remove event file at path {}: {e}", self.0.display()),
        }
    }
}
