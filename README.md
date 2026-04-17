# embassy-ssd1306

Driver asynchrone `no_std` pour l'écran OLED SSD1306 128x64 via I2C.  
Optimisé pour l'exécuteur `embassy`.

# Introduction de #![forbid(unsafe_code)] pour du safety.

## Utilisation

```toml
[dependencies]
embassy-ssd1306 = "0.1.0"
```

```rust
use embassy_ssd1306::Ssd1306;

let mut oled = Ssd1306::new(i2c, 0x3C);
oled.init().await.unwrap();

oled.draw_rect(0, 0, 128, 64, true);
oled.draw_i16(0, 0, -1234);
oled.flush().await.unwrap();
```

## Fonctionnalités

- `draw_pixel` / `draw_hline` / `draw_vline`
- `draw_rect` / `draw_filled_rect`
- `draw_bitmap` (1bpp, MSB à gauche)
- `draw_char` / `draw_i16` (font 5x7, chiffres + signe)
- Framebuffer 1024 bytes en RAM, flush optimisé page par page

## Licence

GPL-2.0-or-later — Copyright (C) 2026 Jorge Andre Castro