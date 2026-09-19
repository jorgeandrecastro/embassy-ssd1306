[![crates.io](https://img.shields.io/crates/v/embassy-ssd1306.svg)](https://crates.io/crates/embassy-ssd1306)
[![docs.rs](https://docs.rs/embassy-ssd1306/badge.svg)](https://docs.rs/embassy-ssd1306)
[![License: GPL v2](https://img.shields.io/badge/License-GPL_v2-blue.svg)](https://www.gnu.org/licenses/old-licenses/gpl-2.0.en.html)

# embassy-ssd1306

Driver asynchrone `no_std` pour l'écran OLED SSD1306 128x64 via I2C, testé sur Pico 2 (RP2350) et Pico (RP2040).
Optimisé pour l'exécuteur `embassy`.

Il fournit un framebuffer en RAM (1024 octets) avec des primitives graphiques
(pixels, lignes, rectangles, bitmaps), du texte et des nombres (police 5x7), et un flush I2C
optimisé page par page (pages 0 à 7).

## Caractères supportés

| Catégorie | Caractères |
|---|---|
| Chiffres et lettres | `0-9`, `A-Z` (les minuscules `a-z` utilisent les mêmes glyphes) |
| Ponctuation | `.` `,` `:` `;` `!` `?` `'` `"` `` ` `` |
| Groupement | `(` `)` `[` `]` `{` `}` `<` `>` |
| Opérateurs ASCII | `+` `-` `=` `*` `/` `\` `\|` `_` `^` `~` `%` |
| Symboles | `#` `@` `&` `$` |
| Maths (Unicode) | `×` `÷` `±` `°` `≤` `≥` `≠` `≈` `√` `∞` `∫` `∂` `→` |
| Grec minuscule (Unicode) | `α` `β` `λ` `θ` `μ` `π` `σ` `φ` `ω` |
| Grec majuscule (Unicode) | `Δ` `Σ` `Ω` |

- Les caractères ASCII s'affichent avec `draw_str(x, page, b"...")`.
- Les symboles Unicode s'affichent avec `draw_text(x, page, "...")`, ou avec `draw_char` et
  les constantes de `font::` (`font::LAMBDA`, `font::THETA`, `font::DOT`…).
- Les autres majuscules grecques (`Π`, `Λ`, `Θ`, `Φ`…) ne sont pas gérées pour l'instant.

## Installation

```toml
[dependencies]
embassy-ssd1306 = "0.7.0"
```

Dépendances du crate :

```toml
embassy-time = "0.5"
embedded-hal-async = "1.0"
```

## Utilisation rapide

```rust
use embassy_ssd1306::Ssd1306;

let mut oled = Ssd1306::new(i2c, 0x3C);
oled.init().await.unwrap();

oled.draw_rect(0, 0, 128, 64, true);
oled.draw_i16(0, 0, -1234);
oled.flush().await.unwrap();
```

## Exemples

### Texte, nombres et symboles

```rust
use embassy_ssd1306::Ssd1306;

let mut oled = Ssd1306::new(i2c, 0x3C);
oled.init().await.unwrap();

// Texte ASCII
oled.draw_str(0, 0, b"HELLO (WORLD)");

// Nombre signé chaîné avec du texte
oled.draw_str(0, 1, b"TEMP: ");
oled.draw_i16(30, 1, -12);
oled.draw_str(45, 1, b"C");

// Caractères additionnels
oled.draw_str(0, 2, b"[STATUS]=OK!");
oled.draw_str(0, 3, b"50% DONE <->");
oled.draw_str(0, 4, b"TEST: A,B,C");

// Maths et lettres grecques (UTF-8)
oled.draw_text(0, 5, "λ=2π/θ  ΔT≤5°");
oled.draw_text(0, 6, "x≠0 → √x ≈ ∞");

// Rectangle de bordure
oled.draw_rect(0, 0, 128, 64, true);

oled.flush().await.unwrap();
```

### Pico (RP2040) : racines carrées

```rust
use embassy_ssd1306::font;

// Initialisation OLED
if let Ok(_) = oled.init().await {
    oled.clear();
    oled.draw_rect(0, 0, 127, 63, true);
    let _ = oled.flush().await;
    Timer::after_millis(500).await;
}

// Affichage avec calcul de racine carrée
let calculs = [4, 16, 25, 23];
let mut idx = 0;

loop {
    oled.clear();
    oled.draw_rect(0, 0, 127, 63, true);

    let n = calculs[idx];
    let res_q15 = sqrt(n);
    let res_humain = (res_q15 as i32 * 181 + 16384) / 32768;

    // Étiquettes
    oled.draw_str(10, 1, b"sqrt");
    oled.draw_str(65, 1, b"Resultat");

    // Nombre et résultat
    let next_x = oled.draw_i16(10, 3, n as i16);
    oled.draw_char(next_x + 5, 3, font::DOT);
    oled.draw_i16(80, 3, res_humain as i16);

    let _ = oled.flush().await;

    idx = (idx + 1) % calculs.len();
    Timer::after_secs(1).await;
}
```

## Fonctionnalités

- `draw_pixel` / `draw_hline` / `draw_vline`
- `draw_rect` / `draw_filled_rect`
- `draw_bitmap` (1bpp, MSB à gauche)
- `draw_char` / `draw_i16` / `draw_str` / `draw_text` (police 5x7 : chiffres, lettres, ponctuation, maths, grec)
- Framebuffer de 1024 octets en RAM, flush optimisé page par page
- Dessin rogné aux bords de l'écran (aucune panique sur coordonnées hors limites)

## Architecture

```
src/
├── lib.rs          constantes, documentation, ré-exports
├── command.rs      opcodes SSD1306 et octets de contrôle I2C
├── font.rs         table des glyphes, constantes, conversion ASCII/Unicode → glyphe
├── framebuffer.rs  FrameBuffer (RAM, indépendant d'I2C) + draw_pixel
├── graphics.rs     lignes, rectangles, bitmaps
├── text.rs         draw_char, draw_i16, draw_str, draw_text
└── driver.rs       Ssd1306<I> : new, init, flush, release
```

Toutes les méthodes de dessin sont sur `FrameBuffer` ; `Ssd1306` s'y déréférence,
donc `oled.draw_rect(...)` fonctionne directement.

## 📋 Historique (Changelog)

Ce projet suit une philosophie pragmatique : chaque mise à jour enrichit les fonctionnalités
tout en minimisant l'empreinte mémoire sur le microcontrôleur.

**v0.7.0** : réorganisation modulaire, 34 nouveaux glyphes (symboles ASCII, opérateurs
mathématiques, lettres grecques), `draw_text` (UTF-8), constantes de glyphes nommées,
corrections de débordements.

**v0.6.0** : nouveaux glyphes `^` `#` `@` `&` `"`.

**v0.5.0** : versions de dépendances explicites pour des builds reproductibles.

**v0.4.0** : opérateurs `+` `/` `|` `_`.

Le détail complet de toutes les versions est dans 👉 [CHANGELOG.md](CHANGELOG.md).

## Licence

GPL-2.0-or-later — Copyright (C) 2026 Jorge Andre Castro