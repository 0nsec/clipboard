//! Backend abstraction for clipboard monitoring and access.

use std::time::SystemTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemType {
    Text,
    Image,
    Html,
    Rtf,
    Files,
    Other,
}

#[derive(Debug, Clone)]
pub struct ClipboardItem {
    pub when: SystemTime,
    pub kind: ItemType,
    pub text: Option<String>,
    pub bytes: Option<Vec<u8>>, // images/rtf/html payloads
}

#[derive(thiserror::Error, Debug)]
pub enum BackendError {
    #[error("backend unavailable: {0}")]
    Unavailable(&'static str),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, BackendError>;

#[allow(async_fn_in_trait)]
pub trait ClipboardBackend: Send + Sync {
    fn name(&self) -> &'static str;
    fn start(&self) -> Result<()>;
    fn stop(&self);
    /// Attempt to read the current clipboard and normalize to ClipboardItem
    fn read_current(&self) -> Result<Option<ClipboardItem>>;
    /// Set the clipboard to the given item
    fn set_clipboard(&self, item: &ClipboardItem) -> Result<()>;
}
