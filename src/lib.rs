
// Copyright (C) 2026 Jorge Andre Castro
// GPL-2.0-or-later

#![no_std]
#![forbid(unsafe_code)]

// ============================================================
// DOCUMENTATION
// ============================================================

//! # embassy-ssd1306
//!
//! Asynchronous `no_std` driver for the SSD1306 128x64 OLED display over I2C.
//!
//! Supports two Embassy time backends:
//!
//! - `time-upstream`: Official Embassy time backend (ARM/Cortex-M).
//! - `time-riscv`: RISC-V backend from the embassy-riscv fork.
//!
//! Select exactly one backend through Cargo features.
//!
//! The crate is organized into independent layers:
//!
//! | Module          | Role                                             |
//! |-----------------|--------------------------------------------------|
//! | [`command`]     | SSD1306 opcodes and I2C control bytes            |
//! | [`font`]        | 5x7 font, glyph indices, ASCII-to-glyph mapping  |
//! | [`framebuffer`] | 1024-byte RAM buffer + `draw_pixel`              |
//! | `graphics`      | Primitives: lines, rectangles, bitmaps           |
//! | `text`          | `draw_char`, `draw_str`, `draw_i16`              |
//! | [`driver`]      | `Ssd1306<I>`: I2C init + flush                   |
//!
//! All drawing methods are implemented on [`FrameBuffer`]; [`Ssd1306`]
//! accesses it through `Deref`, so the `oled.draw_rect(...)` API remains unchanged.
//!
//! # Example
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

// ============================================================
// EMBASSY TIME BACKEND SELECTION
// ============================================================

// RISC-V: expose embassy-time-riscv under the standard crate name.
#[cfg(feature = "time-riscv")]
extern crate embassy_time_riscv as embassy_time;

// Prevent selecting both backends simultaneously.
#[cfg(all(feature = "time-upstream", feature = "time-riscv"))]
compile_error!(
    "Features \"time-upstream\" and \"time-riscv\" are mutually exclusive."
);

// Require exactly one backend.
#[cfg(not(any(feature = "time-upstream", feature = "time-riscv")))]
compile_error!(
    "Select one Embassy time backend: \"time-upstream\" or \"time-riscv\"."
);

// ============================================================
// DISPLAY CONSTANTS
// ============================================================

/// Display width in pixels.
pub const SCREEN_WIDTH: usize = 128;

/// Display height in pixels.
pub const SCREEN_HEIGHT: usize = 64;

/// Number of pages (1 page = 8 pixels high).
pub const PAGES: usize = SCREEN_HEIGHT / 8;

// ============================================================
// MODULES
// ============================================================

pub mod command;
pub mod driver;
pub mod font;
pub mod framebuffer;

mod graphics;
mod text;

// ============================================================
// PUBLIC API
// ============================================================

pub use driver::Ssd1306;
pub use framebuffer::FrameBuffer;
