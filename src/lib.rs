// Copyright (C) 2026 Jorge Andre Castro
// GPL-2.0-or-later

#![no_std]
#![forbid(unsafe_code)]

//! # embassy-ssd1306
//!
//! Driver asynchrone `no_std` pour l'écran OLED SSD1306 128x64 via I2C.
//!
//! Le crate est organisé en couches indépendantes :
//!
//! | Module          | Rôle                                             |
//! |-----------------|--------------------------------------------------|
//! | [`command`]     | Opcodes SSD1306 et octets de contrôle I2C        |
//! | [`font`]        | Police 5x7, indices de glyphes, ASCII → glyphe   |
//! | [`framebuffer`] | Tampon RAM 1024 octets + `draw_pixel`            |
//! | `graphics`      | Primitives : lignes, rectangles, bitmaps         |
//! | `text`          | `draw_char`, `draw_str`, `draw_i16`              |
//! | [`driver`]      | `Ssd1306<I>` : init + flush I2C                  |
//!
//! Toutes les méthodes de dessin sont sur [`FrameBuffer`] ; [`Ssd1306`]
//! y accède par `Deref`, l'API `oled.draw_rect(...)` reste donc inchangée.
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

/// Largeur de l'écran en pixels.
pub const SCREEN_WIDTH: usize = 128;

/// Hauteur de l'écran en pixels.
pub const SCREEN_HEIGHT: usize = 64;

/// Nombre de pages (1 page = 8 pixels de hauteur).
pub const PAGES: usize = SCREEN_HEIGHT / 8;

pub mod command;
pub mod driver;
pub mod font;
pub mod framebuffer;
mod graphics;
mod text;

pub use driver::Ssd1306;
pub use framebuffer::FrameBuffer;