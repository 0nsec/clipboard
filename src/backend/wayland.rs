//! Wayland backend (stub)

use super::*;

pub struct WaylandBackend;

impl WaylandBackend {
    pub fn new() -> Self { Self }
}

impl ClipboardBackend for WaylandBackend {
    fn name(&self) -> &'static str { "wayland" }
    fn start(&self) -> Result<()> { Ok(()) }
    fn stop(&self) {}
    fn read_current(&self) -> Result<Option<ClipboardItem>> { Ok(None) }
    fn set_clipboard(&self, _item: &ClipboardItem) -> Result<()> { Ok(()) }
}
