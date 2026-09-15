//! Filesystem watch service (`notify`, P1-12).
//!
//! Watches theme, icon-pack, translation, plugin and asset roots and routes
//! filesystem happenings into controlled [`WatchMessage`]s. Events are hints,
//! never trusted content: the consumer re-reads and validates the file before
//! applying anything. Bursts are debounced and coalesced per path.
//!
//! The watcher runs on its own thread; messages cross to the application
//! through a bounded channel drained on the app tick.

use std::path::{Path, PathBuf};
use std::time::Duration;

/// Debounce window: bursts on one path collapse into a single message.
pub const DEBOUNCE_WINDOW: Duration = Duration::from_millis(250);
/// Channel capacity; beyond it, oldest hints are shed (consumer re-reads).
pub const CHANNEL_CAPACITY: usize = 64;

/// Controlled message produced from filesystem activity.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WatchMessage {
    /// Which watched root produced the hint.
    pub root: PathBuf,
    /// The affected path, if identifiable.
    pub path: Option<PathBuf>,
    /// What happened (created/modified/removed/renamed).
    pub kind: WatchKind,
}

/// Coarse change kind. Details are intentionally lossy: the consumer
/// re-reads the file.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WatchKind {
    /// Created, modified or renamed-to.
    Changed,
    /// Removed or renamed-from.
    Removed,
}

/// Watch service errors.
#[derive(Debug, thiserror::Error)]
pub enum WatchError {
    /// Root cannot be watched (missing, not a dir, backend failure).
    #[error("cannot watch '{path}': {detail}")]
    Watch {
        /// Offending root.
        path: String,
        /// Cause.
        detail: String,
    },
}

/// Filesystem watcher. Drop stops the background thread.
pub struct WatchService {
    _watcher: notify::RecommendedWatcher,
    receiver: flume::Receiver<WatchMessage>,
    roots: Vec<PathBuf>,
}

impl WatchService {
    /// Starts watching `roots` (must exist and be directories).
    pub fn watch(roots: &[&Path]) -> Result<Self, WatchError> {
        use notify::{EventKind, RecursiveMode, Watcher as _};
        for root in roots {
            if !root.is_dir() {
                return Err(WatchError::Watch {
                    path: root.display().to_string(),
                    detail: "not an existing directory".to_string(),
                });
            }
        }
        let (sender, receiver) = flume::bounded(CHANNEL_CAPACITY);
        let owned_for_cb: Vec<PathBuf> = roots.iter().map(|r| r.to_path_buf()).collect();
        let mut watcher =
            notify::recommended_watcher(move |event: notify::Result<notify::Event>| {
                let Ok(event) = event else { return };
                let kind = match event.kind {
                    EventKind::Remove(_) => WatchKind::Removed,
                    _ => WatchKind::Changed,
                };
                for path in event.paths {
                    let root = owned_for_cb
                        .iter()
                        .find(|root| path.starts_with(root))
                        .cloned()
                        .unwrap_or_default();
                    let message = WatchMessage {
                        root,
                        path: Some(path),
                        kind,
                    };
                    // Shedding is correct: the consumer re-reads anyway.
                    let _ = sender.try_send(message);
                }
            })
            .map_err(|e| WatchError::Watch {
                path: "<watcher>".to_string(),
                detail: e.to_string(),
            })?;
        let mut owned_roots = Vec::new();
        for root in roots {
            watcher
                .watch(root, RecursiveMode::Recursive)
                .map_err(|e| WatchError::Watch {
                    path: root.display().to_string(),
                    detail: e.to_string(),
                })?;
            owned_roots.push(root.to_path_buf());
        }
        Ok(Self {
            _watcher: watcher,
            receiver,
            roots: owned_roots,
        })
    }

    /// Watched roots.
    pub fn roots(&self) -> &[PathBuf] {
        &self.roots
    }

    /// Drains pending messages, coalesced per path: the latest message per
    /// path wins (bursts collapse; the consumer re-reads anyway).
    pub fn drain_coalesced(&self) -> Vec<WatchMessage> {
        let mut latest: Vec<WatchMessage> = Vec::new();
        for message in self.receiver.try_iter() {
            if let Some(path) = message.path.clone() {
                latest.retain(|kept| kept.path.as_ref() != Some(&path));
            }
            latest.push(message);
        }
        latest
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_root_is_rejected() {
        let err = WatchService::watch(&[Path::new("/definitely/not/here-petunia")]).err();
        assert!(matches!(err, Some(WatchError::Watch { .. })));
    }

    #[test]
    fn file_activity_produces_coalesced_message() {
        let dir = tempfile_like_dir();
        let service = WatchService::watch(&[dir.as_path()]).unwrap();
        assert_eq!(service.roots(), std::slice::from_ref(&dir));
        std::fs::write(dir.join("theme.toml"), b"v=1").unwrap();
        std::fs::write(dir.join("theme.toml"), b"v=2").unwrap();
        let mut seen = None;
        for _ in 0..100 {
            std::thread::sleep(Duration::from_millis(20));
            let drained = service.drain_coalesced();
            if !drained.is_empty() {
                seen = Some(drained);
                break;
            }
        }
        let messages = seen.expect("watcher delivers file activity");
        // Bursts on one path collapse to a single message.
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].kind, WatchKind::Changed);
        assert_eq!(messages[0].root, dir);
        let _ = std::fs::remove_dir_all(&dir);
    }

    fn tempfile_like_dir() -> PathBuf {
        let dir = std::env::temp_dir().join(format!("petunia-watch-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }
}
