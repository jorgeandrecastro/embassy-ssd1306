//! RAM framebuffer, independent of the I2C bus.

use crate::{PAGES, SCREEN_HEIGHT, SCREEN_WIDTH};

/// Framebuffer size in bytes (1024 for 128x64).
pub const FRAMEBUFFER_SIZE: usize = SCREEN_WIDTH * PAGES;

/// 1bpp framebuffer in SSD1306 page format (1 byte = 8 vertical pixels).
///
/// Depends on neither I2C nor Embassy: drawing primitives
/// (`graphics.rs`, `text.rs`) are implemented directly on it.
pub struct FrameBuffer {
    pub(crate) buf: [u8; FRAMEBUFFER_SIZE],
}

impl FrameBuffer {
    /// Creates an empty (black) framebuffer.
    pub const fn new() -> Self {
        Self { buf: [0; FRAMEBUFFER_SIZE] }
    }

    /// Clears the framebuffer (all black).
    pub fn clear(&mut self) {
        self.buf.fill(0x00);
    }

    /// Fills the framebuffer (all white).
    pub fn fill(&mut self) {
        self.buf.fill(0xFF);
    }

    /// Raw framebuffer contents.
    pub fn as_bytes(&self) -> &[u8; FRAMEBUFFER_SIZE] {
        &self.buf
    }

    /// Bytes for a page (0..8), or `None` if out of bounds.
    pub fn page(&self, page: usize) -> Option<&[u8]> {
        self.buf.get(page * SCREEN_WIDTH..(page + 1) * SCREEN_WIDTH)
    }

    /// Turns a pixel on or off. Coordinates outside the display are ignored.
    pub fn draw_pixel(&mut self, x: u8, y: u8, on: bool) {
        if x as usize >= SCREEN_WIDTH || y as usize >= SCREEN_HEIGHT {
            return;
        }
        let idx = (y as usize / 8) * SCREEN_WIDTH + x as usize;
        let mask = 1u8 << (y % 8);
        if on {
            self.buf[idx] |= mask;
        } else {
            self.buf[idx] &= !mask;
        }
    }
}

impl Default for FrameBuffer {
    fn default() -> Self {
        Self::new()
    }
}