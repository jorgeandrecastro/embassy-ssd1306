//! Text and number rendering (5x7 font) on [`FrameBuffer`].

use crate::font::{char_to_glyph, char_to_glyph_unicode, FONT};
use crate::framebuffer::FrameBuffer;
use crate::{PAGES, SCREEN_WIDTH};

/// Character width + 1 spacing column.
const CHAR_ADVANCE: u8 = 6;

impl FrameBuffer {
    /// Draws a 5x7 glyph at position (`x`, `page`).
    ///
    /// `glyph_idx`: see the constants in [`crate::font`].
    /// Invalid indices or pages are ignored; the glyph is clipped at the
    /// right edge of the display (it does not overflow onto the next page).
    pub fn draw_char(&mut self, x: u8, page: u8, glyph_idx: usize) {
        let Some(glyph) = FONT.get(glyph_idx) else { return };
        if page as usize >= PAGES {
            return;
        }
        for (col, &byte) in glyph.iter().enumerate() {
            let px = x as usize + col;
            if px >= SCREEN_WIDTH {
                break;
            }
            self.buf[page as usize * SCREEN_WIDTH + px] = byte;
        }
    }

    /// Displays a signed 16-bit integer. Returns the X coordinate after the last character.
    pub fn draw_i16(&mut self, mut x: u8, page: u8, val: i16) -> u8 {
        if val < 0 {
            self.draw_char(x, page, crate::font::MINUS);
            x = x.saturating_add(CHAR_ADVANCE);
        }

        let mut n = val.unsigned_abs();
        let mut digits = [0u8; 5];
        let mut count = 0;
        loop {
            digits[count] = (n % 10) as u8;
            n /= 10;
            count += 1;
            if n == 0 {
                break;
            }
        }

        for i in (0..count).rev() {
            self.draw_char(x, page, digits[i] as usize);
            x = x.saturating_add(CHAR_ADVANCE);
        }
        x
    }

    /// Displays an ASCII string. Unsupported characters are ignored
    /// (but leave a space). Returns the X coordinate after the last character.
    pub fn draw_str(&mut self, mut x: u8, page: u8, text: &[u8]) -> u8 {
        for &c in text {
            if let Some(idx) = char_to_glyph(c) {
                self.draw_char(x, page, idx);
            }
            x = x.saturating_add(CHAR_ADVANCE);
        }
        x
    }

    /// Displays a UTF-8 string (ASCII + mathematical symbols + Greek letters;
    /// see [`char_to_glyph_unicode`]). Unsupported characters are ignored
    /// (but leave a space). Returns the final X coordinate.
    pub fn draw_text(&mut self, mut x: u8, page: u8, text: &str) -> u8 {
        for c in text.chars() {
            if let Some(idx) = char_to_glyph_unicode(c) {
                self.draw_char(x, page, idx);
            }
            x = x.saturating_add(CHAR_ADVANCE);
        }
        x
    }
}