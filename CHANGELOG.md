# 📜 Changelog - embassy-ssd1306

The primary goal of this driver is to remain lightweight and pragmatic. We avoid
adding complex features that would bloat the binary, preserving the critical
resources of microcontrollers (RAM/Flash).

All notable changes to `embassy-ssd1306` are documented here.
This project follows [Semantic Versioning](https://semver.org/).

## [0.7.4]

- Readme fix and cargo toml fix.

## [0.7.3]

### Changed
- Replaced the local Cargo `[patch.crates-io]` configuration with published
  Crates.io dependencies:
  - `embassy-time-riscv = "0.1.2"`
  - `embassy-time-driver-riscv = "0.1"`
- Kept the upstream `embassy-time = "0.5"` backend available for ARM/Cortex-M.
- Consumers can enable RISC-V support with the `time-riscv` Cargo feature without
  patching `embassy-time` or `embassy-time-driver-riscv` themselves.

## [0.7.2]

### Changed
- Documented RISC-V support using the `embassy-time-riscv` crate dependency.
- Added selectable `time-upstream` and `time-riscv` Cargo features.
- Restored the standard `embassy-time = "0.5"` dependency to avoid native
  `embassy-time` driver conflicts between upstream and RISC-V implementations.

## [0.7.1]

### Changed
- Translated the project documentation into English.

## [0.7.0]

### Added
- **34 new glyphs** (the table grew from 60 to 94 entries):
  - ASCII: `*` `'` `;` `{` `}` `\` `~` `$` `` ` ``
  - Mathematical operators: `×` `÷` `±` `°` `≤` `≥` `≠` `≈` `√` `∞` `∫` `∂` `→`
  - Lowercase Greek letters: `α` `β` `λ` `θ` `μ` `π` `σ` `φ` `ω`
  - Uppercase Greek letters: `Δ` `Σ` `Ω`
- `font::char_to_glyph_unicode(char)`: converts a Unicode character to a glyph index
  (supported variants: `ϑ`, `ϕ`, `µ`).
- `draw_text(x, page, &str)`: displays UTF-8 text (ASCII + symbols + Greek letters).
- Named glyph constants in `font` (`font::DOT`, `font::LAMBDA`, `font::THETA`...)
  for use with `draw_char` instead of numeric indices.
- `Ssd1306::release()` to retrieve the I2C bus.

### Changed
- **Modular reorganization** (public API unchanged; `Ssd1306` dereferences to `FrameBuffer`):
  `command` (opcodes), `font`, `framebuffer`, `graphics`, `text`, `driver`.
- `init()` opcodes are now named (`command::SET_CONTRAST`, `command::CHARGE_PUMP`...).

### Fixed
- Possible `u8` overflow in `draw_hline`, `draw_vline`, and `draw_bitmap` (drawing is now clipped).
- `draw_rect` no longer overflows when `w == 0` or `h == 0`.
- `draw_char`: invalid glyph indices are ignored, and glyphs near the right edge
  no longer overflow onto the next page.

### Note
- Uppercase Greek letters without a dedicated glyph (`Π`, `Λ`, `Θ`, `Φ`...) are not currently supported.

## [0.6.0] - Symbols and formatting

New glyphs: Added 5 new glyphs: `^`, `#`, `@`, `&`, `"`. These make it possible
to display exponent/accent symbols, comments, email addresses, ampersands, and strings.

Feature: Expanded support for annotations, metadata, and specialized characters.

## [0.4.0] - Operator symbols and formatting

Additional symbols: Added 5 new symbols: `+`, `/`, `|`, `_`. These make it possible
to display equations, content separators, underscores, and tildes.

Feature: Full support for simple mathematical calculations, structured formatting,
and text-based visualizations.

## [0.3.0] - Special characters and safety

Additional characters: Added support for 14 new special characters:
`(`, `)`, `,`, `[`, `]`, `%`, `<`, `>`, `=`, `?`, `!`, `:`. This enables richer
and more informative text displays (status messages, percentages, mathematical
expressions, and more).

Documentation improvements: Updated examples to demonstrate the use of the new characters.

## [0.2.4] - Stability update

Embassy flexibility: Migrated to the `embassy-time = ">=0.3, <0.5"` version range.
This allows the driver to be used on Pico 2 (RP2350) and future versions of Embassy
without dependency conflicts.

## [0.2.3] - Hardware example

Pico validation: Added a complete example validated and tested on the Raspberry Pi
Pico (RP2040), providing a quick start with standard wiring.

## [0.2.1] - Punctuation

Added the `.` character for numeric displays and sentence endings.

Design choice: A space is automatically inserted after the period. This deliberate
choice keeps the code simple and efficient, avoiding character-width calculation
functions that would be too costly for the processor.

## [0.2.0] - Text handling

Letter display: Introduced alphabetic character support.

Pragmatism: Text is displayed in uppercase by default. This simplifies the
implementation, maximizes readability on small OLED screens, and minimizes
the font's footprint in Flash memory.
