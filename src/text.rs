//! Rendu de texte et de nombres (police 5x7) sur [`FrameBuffer`].

use crate::font::{char_to_glyph, char_to_glyph_unicode, FONT};
use crate::framebuffer::FrameBuffer;
use crate::{PAGES, SCREEN_WIDTH};

/// Largeur d'un caractère + 1 colonne d'espacement.
const CHAR_ADVANCE: u8 = 6;

impl FrameBuffer {
    /// Dessine un glyphe 5x7 à la position (`x`, `page`).
    ///
    /// `glyph_idx` : voir les constantes de [`crate::font`].
    /// Un index ou une page invalide est ignoré ; le glyphe est rogné
    /// au bord droit de l'écran (pas de débordement sur la page suivante).
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

    /// Affiche un entier signé 16 bits. Retourne le X après le dernier caractère.
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

    /// Affiche une chaîne ASCII. Les caractères non supportés sont ignorés
    /// (mais laissent un espace). Retourne le X après le dernier caractère.
    pub fn draw_str(&mut self, mut x: u8, page: u8, text: &[u8]) -> u8 {
        for &c in text {
            if let Some(idx) = char_to_glyph(c) {
                self.draw_char(x, page, idx);
            }
            x = x.saturating_add(CHAR_ADVANCE);
        }
        x
    }

    /// Affiche une chaîne UTF-8 (ASCII + symboles mathématiques + lettres
    /// grecques, voir [`char_to_glyph_unicode`]). Les caractères non supportés
    /// sont ignorés (mais laissent un espace). Retourne le X final.
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