//! X11 backend (stub)

use super::*;

pub struct X11Backend;

impl X11Backend {
    pub fn new() -> Self { Self }
}

impl ClipboardBackend for X11Backend {
    fn name(&self) -> &'static str { "x11" }
    fn start(&self) -> Result<()> { Ok(()) }
    fn stop(&self) {}
    fn read_current(&self) -> Result<Option<ClipboardItem>> { Ok(None) }
    fn set_clipboard(&self, _item: &ClipboardItem) -> Result<()> { Ok(()) }
}
