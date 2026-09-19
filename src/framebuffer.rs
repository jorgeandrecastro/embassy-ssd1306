//! Tampon d'image en RAM, indépendant du bus I2C.

use crate::{PAGES, SCREEN_HEIGHT, SCREEN_WIDTH};

/// Taille du framebuffer en octets (1024 pour 128x64).
pub const FRAMEBUFFER_SIZE: usize = SCREEN_WIDTH * PAGES;

/// Framebuffer 1bpp au format pages SSD1306 (1 octet = 8 pixels verticaux).
///
/// Ne dépend ni d'I2C ni d'Embassy : les primitives de dessin
/// (`graphics.rs`, `text.rs`) sont implémentées directement dessus.
pub struct FrameBuffer {
    pub(crate) buf: [u8; FRAMEBUFFER_SIZE],
}

impl FrameBuffer {
    /// Crée un framebuffer vide (noir).
    pub const fn new() -> Self {
        Self { buf: [0; FRAMEBUFFER_SIZE] }
    }

    /// Efface le framebuffer (tout noir).
    pub fn clear(&mut self) {
        self.buf.fill(0x00);
    }

    /// Remplit le framebuffer (tout blanc).
    pub fn fill(&mut self) {
        self.buf.fill(0xFF);
    }

    /// Contenu brut du framebuffer.
    pub fn as_bytes(&self) -> &[u8; FRAMEBUFFER_SIZE] {
        &self.buf
    }

    /// Octets d'une page (0..8), ou `None` si hors limites.
    pub fn page(&self, page: usize) -> Option<&[u8]> {
        self.buf.get(page * SCREEN_WIDTH..(page + 1) * SCREEN_WIDTH)
    }

    /// Allume ou éteint un pixel. Les coordonnées hors écran sont ignorées.
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