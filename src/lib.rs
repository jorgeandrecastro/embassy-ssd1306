// Copyright (C) 2026 Jorge Andre Castro
// GPL-2.0-or-later

#![no_std]
#![forbid(unsafe_code)]

//! # embassy-ssd1306
//!
//! Driver asynchrone `no_std` pour l'écran OLED SSD1306 128x64 via I2C.
//! Attention pour l'instant il est possible que d'afficher des nombres 
// sur les pages 0 à 7 prochain update les lettres .Optimisé pour l'exécuteur `embassy`.
//!
//! Ce pilote fournit un framebuffer en RAM avec des primitives graphiques
//! (pixels, lignes, rectangles, bitmaps, texte numérique) et un flush I2C
//! optimisé page par page.
//!
//! # Exemple
//!
//! ```rust,no_run
//! use embassy_ssd1306::Ssd1306;
//!
//! let mut oled = Ssd1306::new(i2c, 0x3C);
//! oled.init().await.unwrap();
//!
//! oled.draw_rect(0, 0, 128, 64, true);
//! oled.draw_i16(0, 0, -1234);
//! oled.flush().await.unwrap();
//! ```

use embassy_time::Timer;
use embedded_hal_async::i2c::I2c;

/// Largeur de l'écran en pixels.
pub const SCREEN_WIDTH: usize = 128;

/// Hauteur de l'écran en pixels.
pub const SCREEN_HEIGHT: usize = 64;

/// Nombre de pages (1 page = 8 pixels de hauteur).
pub const PAGES: usize = SCREEN_HEIGHT / 8;

/// Font 5x7 — chiffres 0-9, signe moins, espace
const FONT: [[u8; 5]; 12] = [
    [0x3E, 0x51, 0x49, 0x45, 0x3E], // 0
    [0x00, 0x42, 0x7F, 0x40, 0x00], // 1
    [0x42, 0x61, 0x51, 0x49, 0x46], // 2
    [0x21, 0x41, 0x45, 0x4B, 0x31], // 3
    [0x18, 0x14, 0x12, 0x7F, 0x10], // 4
    [0x27, 0x45, 0x45, 0x45, 0x39], // 5
    [0x3C, 0x4A, 0x49, 0x49, 0x30], // 6
    [0x01, 0x71, 0x09, 0x05, 0x03], // 7
    [0x36, 0x49, 0x49, 0x49, 0x36], // 8
    [0x06, 0x49, 0x49, 0x29, 0x1E], // 9
    [0x08, 0x08, 0x08, 0x08, 0x08], // 10 = '-'
    [0x00, 0x00, 0x00, 0x00, 0x00], // 11 = ' '
];

/// Instance principale du driver SSD1306.
///
/// Fonctionne avec n'importe quel périphérique implémentant
/// `embedded-hal-async::i2c::I2c`.
pub struct Ssd1306<I: I2c> {
    i2c: I,
    /// Adresse I2C configurée (0x3C ou 0x3D).
    pub addr: u8,
    framebuffer: [u8; SCREEN_WIDTH * PAGES],
}

impl<I: I2c> Ssd1306<I> {
    /// Initialise une nouvelle instance du driver.
    ///
    /// # Arguments
    /// * `i2c`  Bus I2C (ou I2cDevice partagé).
    /// * `addr`  Adresse du composant (généralement 0x3C).
    pub fn new(i2c: I, addr: u8) -> Self {
        Self {
            i2c,
            addr,
            framebuffer: [0u8; SCREEN_WIDTH * PAGES],
        }
    }

    // Commandes bas niveau 

    async fn cmd(&mut self, c: u8) -> Result<(), I::Error> {
        self.i2c.write(self.addr, &[0x00, c]).await
    }

    async fn cmd2(&mut self, c: u8, d: u8) -> Result<(), I::Error> {
        self.i2c.write(self.addr, &[0x00, c, d]).await
    }

    async fn cmd3(&mut self, c: u8, d1: u8, d2: u8) -> Result<(), I::Error> {
        self.i2c.write(self.addr, &[0x00, c, d1, d2]).await
    }

    // Init

    /// Configure le SSD1306 et efface l'écran.
    ///
    /// Doit être appelé une fois avant toute opération d'affichage.
    pub async fn init(&mut self) -> Result<(), I::Error> {
        Timer::after_millis(200).await;

        self.cmd(0xAE).await?;           // Display OFF
        self.cmd2(0xD5, 0x80).await?;    // Osc freq
        self.cmd2(0xA8, 0x3F).await?;    // Multiplex 64
        self.cmd2(0xD3, 0x00).await?;    // Display offset 0
        self.cmd(0x40).await?;           // Start line 0
        self.cmd2(0x8D, 0x14).await?;    // Charge pump ON
        self.cmd2(0x20, 0x00).await?;    // Horizontal addressing mode
        self.cmd(0xA1).await?;           // Segment remap
        self.cmd(0xC8).await?;           // COM scan dec
        self.cmd2(0xDA, 0x12).await?;    // COM pins
        self.cmd2(0x81, 0xCF).await?;    // Contrast
        self.cmd2(0xD9, 0xF1).await?;    // Pre-charge
        self.cmd2(0xDB, 0x40).await?;    // VCOMH
        self.cmd(0xA4).await?;           // RAM display
        self.cmd(0xA6).await?;           // Normal (non-inversé)
        self.cmd(0xAF).await?;           // Display ON

        self.clear();
        self.flush().await
    }

    // Framebuffer 

    /// Efface le framebuffer (tout noir). Ne flush pas vers l'écran.
    pub fn clear(&mut self) {
        self.framebuffer.fill(0x00);
    }

    /// Remplit le framebuffer (tout blanc). Ne flush pas vers l'écran.
    pub fn fill(&mut self) {
        self.framebuffer.fill(0xFF);
    }

    /// Allume ou éteint un pixel dans le framebuffer.
    ///
    /// Les coordonnées hors écran sont ignorées silencieusement.
    pub fn draw_pixel(&mut self, x: u8, y: u8, on: bool) {
        if x >= SCREEN_WIDTH as u8 || y >= SCREEN_HEIGHT as u8 {
            return;
        }
        let page = (y / 8) as usize;
        let bit = y % 8;
        let idx = page * SCREEN_WIDTH + x as usize;
        if on {
            self.framebuffer[idx] |= 1 << bit;
        } else {
            self.framebuffer[idx] &= !(1 << bit);
        }
    }

    //  Primitives graphiques 

    /// Dessine une ligne horizontale.
    pub fn draw_hline(&mut self, x: u8, y: u8, w: u8, on: bool) {
        for i in 0..w {
            self.draw_pixel(x + i, y, on);
        }
    }

    /// Dessine une ligne verticale.
    pub fn draw_vline(&mut self, x: u8, y: u8, h: u8, on: bool) {
        for i in 0..h {
            self.draw_pixel(x, y + i, on);
        }
    }

    /// Dessine un rectangle vide.
    pub fn draw_rect(&mut self, x: u8, y: u8, w: u8, h: u8, on: bool) {
        self.draw_hline(x,         y,         w, on);
        self.draw_hline(x,         y + h - 1, w, on);
        self.draw_vline(x,         y,         h, on);
        self.draw_vline(x + w - 1, y,         h, on);
    }

    /// Dessine un rectangle plein.
    pub fn draw_filled_rect(&mut self, x: u8, y: u8, w: u8, h: u8, on: bool) {
        for row in 0..h {
            self.draw_hline(x, y + row, w, on);
        }
    }

    /// Dessine un bitmap 1bpp (MSB à gauche).
    ///
    /// # Arguments
    /// * `data`  Chaque byte représente 8 pixels horizontaux consécutifs.
    pub fn draw_bitmap(&mut self, x: u8, y: u8, w: u8, h: u8, data: &[u8]) {
        let stride = (w as usize + 7) / 8;
        for row in 0..h as usize {
            for col in 0..w as usize {
                let byte_idx = row * stride + col / 8;
                let bit = 7 - (col % 8);
                let on = byte_idx < data.len() && (data[byte_idx] >> bit) & 1 == 1;
                self.draw_pixel(x + col as u8, y + row as u8, on);
            }
        }
    }

    // Texte 

    /// Dessine un glyphe 5x7 dans le framebuffer à la position (x, page).
    /// `glyph_idx` : index dans la table FONT (0-9 = chiffres, 10 = '-', 11 = ' ').
    pub fn draw_char(&mut self, x: u8, page: u8, glyph_idx: usize) {
        for col in 0..5usize {
            let byte = FONT[glyph_idx][col];
            let fb_idx = page as usize * SCREEN_WIDTH + x as usize + col;
            if fb_idx < self.framebuffer.len() {
                self.framebuffer[fb_idx] = byte;
            }
        }
    }

    /// Affiche un entier signé 16 bits à la position (x, page).
    /// Retourne la coordonnée X après le dernier caractère écrit,
    /// ce qui permet de chaîner plusieurs valeurs sur la même ligne.
    pub fn draw_i16(&mut self, mut x: u8, page: u8, val: i16) -> u8 {
        if val < 0 {
            self.draw_char(x, page, 10); // '-'
            x += 6;
        }

        let mut n = val.unsigned_abs();
        let mut digits = [0u8; 5];
        let mut count = 0;

        loop {
            digits[count] = (n % 10) as u8;
            n /= 10;
            count += 1;
            if n == 0 { break; }
        }

        for i in (0..count).rev() {
            self.draw_char(x, page, digits[i] as usize);
            x += 6;
        }
        x
    }

    // Flush
    /// Envoie le framebuffer complet vers l'écran via I2C.
    ///
    /// À appeler après toutes les opérations de dessin pour rendre
    /// les modifications visibles.
    pub async fn flush(&mut self) -> Result<(), I::Error> {
        self.cmd3(0x21, 0, 127).await?; // colonnes 0..127
        self.cmd3(0x22, 0, 7).await?;   // pages 0..7

        let mut buf = [0u8; 129];
        buf[0] = 0x40; // Co=0, D/C#=1 → mode DATA
        for page in 0..PAGES {
            let start = page * SCREEN_WIDTH;
            buf[1..129].copy_from_slice(&self.framebuffer[start..start + SCREEN_WIDTH]);
            self.i2c.write(self.addr, &buf).await?;
        }
        Ok(())
    }
}