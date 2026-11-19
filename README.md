[![crates.io](https://img.shields.io/crates/v/embassy-ssd1306.svg)](https://crates.io/crates/embassy-ssd1306)
[![docs.rs](https://docs.rs/embassy-ssd1306/badge.svg)](https://docs.rs/embassy-ssd1306)
[![License: GPL v2](https://img.shields.io/badge/License-GPL_v2-blue.svg)](https://www.gnu.org/licenses/old-licenses/gpl-2.0.en.html)

# embassy-ssd1306

Asynchronous `no_std` driver for the SSD1306 128x64 OLED display over I2C, tested on the Pico 2 (RP2350) and Pico (RP2040).
Optimized for the `embassy` executor.

It provides a 1024-byte RAM framebuffer with graphics primitives
(pixels, lines, rectangles, bitmaps), text and numbers (5x7 font), and an
optimized page-by-page I2C flush (pages 0 through 7).

## Supported characters

| Category | Characters |
|---|---|
| Digits and letters | `0-9`, `A-Z` (lowercase `a-z` use the same glyphs) |
| Punctuation | `.` `,` `:` `;` `!` `?` `'` `"` `` ` `` |
| Grouping | `(` `)` `[` `]` `{` `}` `<` `>` |
| ASCII operators | `+` `-` `=` `*` `/` `\` `\|` `_` `^` `~` `%` |
| Symbols | `#` `@` `&` `$` |
| Math (Unicode) | `×` `÷` `±` `°` `≤` `≥` `≠` `≈` `√` `∞` `∫` `∂` `→` |
| Lowercase Greek (Unicode) | `α` `β` `λ` `θ` `μ` `π` `σ` `φ` `ω` |
| Uppercase Greek (Unicode) | `Δ` `Σ` `Ω` |

- ASCII characters are displayed with `draw_str(x, page, b"...")`.
- Unicode symbols are displayed with `draw_text(x, page, "...")`, or with `draw_char` and
  the constants from `font::` (`font::LAMBDA`, `font::THETA`, `font::DOT`...).
- Other uppercase Greek letters (`Π`, `Λ`, `Θ`, `Φ`...) are not currently supported.


## Architecture and Embassy time backend

The driver supports ARM/Cortex-M and RISC-V targets. Select the Embassy time
backend that matches your target:

| Target family | Cargo feature | Backend |
|---|---|---|
| ARM / Cortex-M | `time-upstream` (default) | Official `embassy-time` |
| RISC-V | `time-riscv` | `embassy-time-riscv` |

For ARM / Cortex-M, the default dependency configuration is sufficient:

```toml
[dependencies]
embassy-ssd1306 = "0.7"
```

For RISC-V, disable the default backend and enable `time-riscv`:

```toml
[dependencies]
embassy-ssd1306 = { version = "0.7", default-features = false, features = ["time-riscv"] }
```

Exactly one time backend must be enabled. For example, check an ARM build with:

```sh
cargo check --target thumbv7em-none-eabihf
```

And a RISC-V build with:

```sh
cargo check --target riscv32imac-unknown-none-elf \
  --no-default-features --features time-riscv
```

## Quick start

```rust
use embassy_ssd1306::Ssd1306;

let mut oled = Ssd1306::new(i2c, 0x3C);
oled.init().await.unwrap();

oled.draw_rect(0, 0, 128, 64, true);
oled.draw_i16(0, 0, -1234);
oled.flush().await.unwrap();
```

## Examples

### Text, numbers, and symbols

```rust
use embassy_ssd1306::Ssd1306;

let mut oled = Ssd1306::new(i2c, 0x3C);
oled.init().await.unwrap();

// ASCII text
oled.draw_str(0, 0, b"HELLO (WORLD)");

// Signed number combined with text
oled.draw_str(0, 1, b"TEMP: ");
oled.draw_i16(30, 1, -12);
oled.draw_str(45, 1, b"C");

// Additional characters
oled.draw_str(0, 2, b"[STATUS]=OK!");
oled.draw_str(0, 3, b"50% DONE <->");
oled.draw_str(0, 4, b"TEST: A,B,C");

// Math symbols and Greek letters (UTF-8)
oled.draw_text(0, 5, "λ=2π/θ  ΔT≤5°");
oled.draw_text(0, 6, "x≠0 → √x ≈ ∞");

// Border rectangle
oled.draw_rect(0, 0, 128, 64, true);

oled.flush().await.unwrap();
```

### Pico (RP2040): square roots

```rust
use embassy_ssd1306::font;

// OLED initialization
if let Ok(_) = oled.init().await {
    oled.clear();
    oled.draw_rect(0, 0, 127, 63, true);
    let _ = oled.flush().await;
    Timer::after_millis(500).await;
}

// Display square root calculations
let calculations = [4, 16, 25, 23];
let mut idx = 0;

loop {
    oled.clear();
    oled.draw_rect(0, 0, 127, 63, true);

    let n = calculations[idx];
    let res_q15 = sqrt(n);
    let res_human = (res_q15 as i32 * 181 + 16384) / 32768;

    // Labels
    oled.draw_str(10, 1, b"sqrt");
    oled.draw_str(65, 1, b"Result");

    // Number and result
    let next_x = oled.draw_i16(10, 3, n as i16);
    oled.draw_char(next_x + 5, 3, font::DOT);
    oled.draw_i16(80, 3, res_human as i16);

    let _ = oled.flush().await;

    idx = (idx + 1) % calculations.len();
    Timer::after_secs(1).await;
}
```

## Features

- `draw_pixel` / `draw_hline` / `draw_vline`
- `draw_rect` / `draw_filled_rect`
- `draw_bitmap` (1bpp, MSB on the left)
- `draw_char` / `draw_i16` / `draw_str` / `draw_text` (5x7 font: digits, letters, punctuation, math symbols, Greek)
- 1024-byte RAM framebuffer, optimized page-by-page flush
- Drawing is clipped to the screen edges (out-of-bounds coordinates never cause a panic)

## Architecture

```
src/
├── lib.rs          constants, documentation, re-exports
├── command.rs      SSD1306 opcodes and I2C control bytes
├── font.rs         glyph table, constants, ASCII/Unicode-to-glyph conversion
├── framebuffer.rs  FrameBuffer (RAM, independent of I2C) + draw_pixel
├── graphics.rs     lines, rectangles, bitmaps
├── text.rs         draw_char, draw_i16, draw_str, draw_text
└── driver.rs       Ssd1306<I>: new, init, flush, release
```

All drawing methods are implemented on `FrameBuffer`; `Ssd1306` dereferences
to it, so `oled.draw_rect(...)` works directly.

## 📋 Changelog

This project follows a pragmatic approach: each update adds useful functionality
while minimizing memory usage on the microcontroller.

**v0.7.4**: Readme fix and cargo toml fix.

**v0.7.3**: RISC-V support now uses the published `embassy-time-riscv` and
`embassy-time-driver-riscv` crates from crates.io, instead of a local Cargo patch.

The full changelog for all versions is available in 👉 [CHANGELOG.md](CHANGELOG.md).

## License

GPL-2.0-or-later 

Copyright (C) 2026 Jorge Andre Castro
