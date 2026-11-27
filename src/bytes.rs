//! The run's owner of file bytes: every reader a verb runs asks one [`Bytes`] for a
//! file, which opens it on first use and hands every later reader the same bytes. A
//! write through the owner is read back through it.
//!
//! `open` is the crate's one counted door onto a file's bytes, so the per-path open
//! count ([`paths_opened`]) is the pin a verb's "no file read twice" holds by, whichever
//! reader reached the disk.

use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use std::io;
use std::path::{Component, Path, PathBuf};
use std::rc::Rc;

thread_local! {
    /// Per-thread count of opens per path through [`open`]. Reads run on their caller's
    /// thread, so a concurrent run on another test thread cannot perturb it.
    static PATHS_OPENED: RefCell<BTreeMap<PathBuf, usize>> = const { RefCell::new(BTreeMap::new()) };
}

/// This thread's cumulative open count per path. Snapshot it before and after a verb
/// run and compare, pinning that the run opens each file at most once.
#[must_use]
pub fn paths_opened() -> BTreeMap<PathBuf, usize> {
    PATHS_OPENED.with(|map| map.borrow().clone())
}

/// The per-path opens between two [`paths_opened`] snapshots, absent paths omitted.
#[must_use]
pub fn opened_since(before: &BTreeMap<PathBuf, usize>) -> BTreeMap<PathBuf, usize> {
    paths_opened()
        .into_iter()
        .filter_map(|(path, after)| {
            let delta = after - before.get(&path).copied().unwrap_or(0);
            (delta > 0).then_some((path, delta))
        })
        .collect()
}

/// A path with its `.` segments dropped, so `./a/b` and `a/b` key one file.
fn key(path: &Path) -> PathBuf {
    path.components()
        .filter(|c| !matches!(c, Component::CurDir))
        .collect()
}

/// Read `path`'s bytes from disk, counting the open. Every file read the crate makes on
/// a verb's path comes through here, directly or through a [`Bytes`] owner.
///
/// # Errors
///
/// The I/O error the read raises.
pub(crate) fn open(path: impl AsRef<Path>) -> io::Result<Vec<u8>> {
    let path = path.as_ref();
    PATHS_OPENED.with(|map| *map.borrow_mut().entry(key(path)).or_default() += 1);
    std::fs::read(path)
}

/// [`open`], decoded as UTF-8; undecodable bytes are an [`io::ErrorKind::InvalidData`].
///
/// # Errors
///
/// The I/O error the read raises, or the decode failure.
pub(crate) fn open_to_string(path: impl AsRef<Path>) -> io::Result<String> {
    String::from_utf8(open(path)?).map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))
}

/// A read's outcome as the owner keeps it: the bytes, or the error's kind and message
/// (an [`io::Error`] is not `Clone`, so each later reader gets a rebuilt one).
type Held = Result<Rc<[u8]>, (io::ErrorKind, String)>;

/// One verb run's owner of file bytes: each path is opened once, on first use, and
/// every later read of it — by any reader the owner is threaded through — is served
/// from what the first read saw.
#[derive(Debug, Default)]
pub struct Bytes {
    held: RefCell<HashMap<PathBuf, Held>>,
}

impl Bytes {
    /// An empty owner; it opens nothing until a reader asks.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// `path`'s bytes, opened on first use.
    ///
    /// # Errors
    ///
    /// The I/O error the first read raised, rebuilt for each reader.
    pub fn read(&self, path: &Path) -> io::Result<Rc<[u8]>> {
        let key = key(path);
        let held = self
            .held
            .borrow_mut()
            .entry(key)
            .or_insert_with(|| {
                open(path)
                    .map(Rc::from)
                    .map_err(|err| (err.kind(), err.to_string()))
            })
            .clone();
        held.map_err(|(kind, message)| io::Error::new(kind, message))
    }

    /// `path`'s bytes decoded as UTF-8; undecodable bytes are an
    /// [`io::ErrorKind::InvalidData`].
    ///
    /// # Errors
    ///
    /// The I/O error the first read raised, or the decode failure.
    pub fn read_to_string(&self, path: &Path) -> io::Result<String> {
        let bytes = self.read(path)?;
        String::from_utf8(bytes.to_vec())
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))
    }

    /// `path`'s text, in the read vocabulary each document format maps into its own.
    pub(crate) fn read_utf8(&self, path: &Path) -> Result<String, crate::hash::ReadUtf8Error> {
        use crate::hash::ReadUtf8Error;
        let bytes = self.read(path).map_err(|source| ReadUtf8Error::Io {
            path: path.to_path_buf(),
            source,
        })?;
        String::from_utf8(bytes.to_vec()).map_err(|source| ReadUtf8Error::NotUtf8 {
            path: path.to_path_buf(),
            source,
        })
    }

    /// Write `bytes` to `path`, creating missing parents, and hold them as the path's
    /// bytes so a later read sees this write without reopening the file.
    ///
    /// # Errors
    ///
    /// The I/O error the write raises; the owner then forgets the path.
    pub fn write(&self, path: &Path, bytes: &[u8]) -> io::Result<()> {
        let key = key(path);
        match crate::fs_util::write_creating_parents(path, bytes) {
            Ok(()) => {
                self.held.borrow_mut().insert(key, Ok(Rc::from(bytes)));
                Ok(())
            }
            Err(err) => {
                self.held.borrow_mut().remove(&key);
                Err(err)
            }
        }
    }

    /// Drop what the owner holds for `path`, so the next read reopens it — for a path
    /// something other than this owner wrote.
    pub fn forget(&self, path: &Path) {
        self.held.borrow_mut().remove(&key(path));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_path_is_opened_once_and_its_writes_read_back() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("a.md");
        std::fs::write(&path, "one").unwrap();
        let bytes = Bytes::new();
        let before = paths_opened();
        assert_eq!(bytes.read_to_string(&path).unwrap(), "one");
        assert_eq!(bytes.read_to_string(&path).unwrap(), "one");
        bytes.write(&path, b"two").unwrap();
        assert_eq!(bytes.read_to_string(&path).unwrap(), "two");
        assert_eq!(opened_since(&before).get(&key(&path)), Some(&1));
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "two");
    }

    #[test]
    fn an_absent_path_is_held_as_its_error() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("missing.md");
        let bytes = Bytes::new();
        let before = paths_opened();
        assert_eq!(
            bytes.read(&path).unwrap_err().kind(),
            io::ErrorKind::NotFound
        );
        assert_eq!(
            bytes.read(&path).unwrap_err().kind(),
            io::ErrorKind::NotFound
        );
        assert_eq!(opened_since(&before).get(&key(&path)), Some(&1));
    }
}
