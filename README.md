
[![crates.io](https://img.shields.io/crates/v/embassy-ssd1306.svg)](https://crates.io/crates/embassy-ssd1306)
[![docs.rs](https://docs.rs/embassy-ssd1306/badge.svg)](https://docs.rs/embassy-ssd1306)
[![License: GPL v2](https://img.shields.io/badge/License-GPL_v2-blue.svg)](https://www.gnu.org/licenses/old-licenses/gpl-2.0.en.html)


# embassy-ssd1306

Driver asynchrone `no_std` pour l'écran OLED SSD1306 128x64 via I2C, testé sur la pico 2 et pico 2040.
Permet d'afficher des nombres, du texte ASCII (A–Z, 0–9) et des caractères spéciaux sur les pages 0 à 7 et l'semble de  **Caractères supportés** : `0-9`, `A-Z`, `.`, `(`, `)`, `,`, `[`, `]`, `%`, `<`, `>`, `=`, `?`, `!`, `:`, `+`, `/`, `|`, `_` 

Ce pilote fournit un framebuffer en RAM avec des primitives graphiques
(pixels, lignes, rectangles, bitmaps, texte numérique) et un flush I2C
optimisé page par page de 0 à 7 pages.
Optimisé pour l'exécuteur `embassy`.

# 📋 Historique et Évolutions (Changelog)
Ce projet suit une philosophie de développement pragmatique : chaque mise à jour vise à enrichir les fonctionnalités tout en minimisant l'empreinte mémoire sur le microcontrôleur.

**Dernière version : v0.4.0**  Caractères étendus et opérateurs pour les calculs et formatage

Ajout de 5 nouveaux symboles opérateurs : `+`, `/`, `|`, `_`

Pour consulter le détail de toutes les versions, veuillez vous référer au fichier :
👉 CHANGELOG.md

## Utilisation

```toml
[dependencies]
embassy-ssd1306 = "0.4.0"
```

```rust
use embassy_ssd1306::Ssd1306;

let mut oled = Ssd1306::new(i2c, 0x3C);
oled.init().await.unwrap();

oled.draw_rect(0, 0, 128, 64, true);
oled.draw_i16(0, 0, -1234);
oled.flush().await.unwrap();
```

----

# Exemple d'utilisation

```rust
use embassy_ssd1306::Ssd1306;

let mut oled = Ssd1306::new(i2c, 0x3C);
oled.init().await.unwrap();

// Texte avec caractères spéciaux
oled.draw_str(0, 0, b"HELLO (WORLD)");

// Nombre signé avec caractères
oled.draw_str(0, 1, b"TEMP: ");
oled.draw_i16(30, 1, -12);
oled.draw_str(45, 1, b"C");

// Exemples avec caractères additionnels
oled.draw_str(0, 2, b"[STATUS]=OK!");
oled.draw_str(0, 3, b"50% DONE <->");
oled.draw_str(0, 4, b"TEST: A,B,C");

// Rectangle de bordure
oled.draw_rect(0, 0, 128, 64, true);

oled.flush().await.unwrap();
```

**Caractères supportés** : `0-9`, `A-Z`, `.`, `(`, `)`, `,`, `[`, `]`, `%`, `<`, `>`, `=`, `?`, `!`, `:`, `+`, `/`, `|`, `_`

---

**Exemple pico 2040**
````rust
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

    // Affichage des étiquettes
    oled.draw_str(10, 1, b"sqrt");
    oled.draw_str(65, 1, b"Resultat");

    // Affichage du nombre et du résultat
    let next_x = oled.draw_i16(10, 3, n as i16);
    oled.draw_char(next_x + 5, 3, 38); // Point '.' (index 38)
    oled.draw_i16(80, 3, res_humain as i16);

    let _ = oled.flush().await;

    idx = (idx + 1) % calculs.len();
    Timer::after_secs(1).await;
}
````

----

## Fonctionnalités

- `draw_pixel` / `draw_hline` / `draw_vline`
- `draw_rect` / `draw_filled_rect`
- `draw_bitmap` (1bpp, MSB à gauche)
- `draw_char` / `draw_i16` / `draw_str` (font 5x7, chiffres, signe, lettres A–Z)
- Framebuffer 1024 bytes en RAM, flush optimisé page par page

----

## Licence

GPL-2.0-or-later — Copyright (C) 2026 Jorge Andre Castro