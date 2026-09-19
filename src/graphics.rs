//! Primitives graphiques sur [`FrameBuffer`].
//!
//! Les additions de coordonnées sont saturantes : un dessin qui dépasse
//! l'écran est simplement rogné, sans panique ni débordement.

use crate::framebuffer::FrameBuffer;

impl FrameBuffer {
    /// Dessine une ligne horizontale.
    pub fn draw_hline(&mut self, x: u8, y: u8, w: u8, on: bool) {
        for i in 0..w {
            self.draw_pixel(x.saturating_add(i), y, on);
        }
    }

    /// Dessine une ligne verticale.
    pub fn draw_vline(&mut self, x: u8, y: u8, h: u8, on: bool) {
        for i in 0..h {
            self.draw_pixel(x, y.saturating_add(i), on);
        }
    }

    /// Dessine un rectangle vide.
    pub fn draw_rect(&mut self, x: u8, y: u8, w: u8, h: u8, on: bool) {
        if w == 0 || h == 0 {
            return;
        }
        self.draw_hline(x, y, w, on);
        self.draw_hline(x, y.saturating_add(h - 1), w, on);
        self.draw_vline(x, y, h, on);
        self.draw_vline(x.saturating_add(w - 1), y, h, on);
    }

    /// Dessine un rectangle plein.
    pub fn draw_filled_rect(&mut self, x: u8, y: u8, w: u8, h: u8, on: bool) {
        for row in 0..h {
            self.draw_hline(x, y.saturating_add(row), w, on);
        }
    }

    /// Dessine un bitmap 1bpp (MSB à gauche).
    ///
    /// `data` : chaque octet représente 8 pixels horizontaux consécutifs.
    pub fn draw_bitmap(&mut self, x: u8, y: u8, w: u8, h: u8, data: &[u8]) {
        let stride = (w as usize + 7) / 8;
        for row in 0..h as usize {
            for col in 0..w as usize {
                let byte_idx = row * stride + col / 8;
                let bit = 7 - (col % 8);
                let on = byte_idx < data.len() && (data[byte_idx] >> bit) & 1 == 1;
                self.draw_pixel(
                    x.saturating_add(col as u8),
                    y.saturating_add(row as u8),
                    on,
                );
            }
        }
    }
}