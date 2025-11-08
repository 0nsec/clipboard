//! Storage layer stubs (SQLite + object store planned)

pub struct Store;

impl Store {
    pub fn open_default() -> anyhow::Result<Self> { Ok(Self) }
}
