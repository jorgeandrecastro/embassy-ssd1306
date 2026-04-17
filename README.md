# embassy-ssd1306

Driver asynchrone `no_std` pour l'écran OLED SSD1306 128x64 via I2C.
Permet d'afficher des nombres et du texte ASCII (A–Z, 0–9) sur les pages 0 à 7.
Ce pilote fournit un framebuffer en RAM avec des primitives graphiques
(pixels, lignes, rectangles, bitmaps, texte numérique) et un flush I2C
optimisé page par page.
Optimisé pour l'exécuteur `embassy`.


# Update La version 0.2.1 introduit le caractere .
Il y aura un espace après le point c'est volontaire pour pas complexifier les fonctions .

# Update La version 0.2.0 introduit l'affichage des lettres .
Par défaut en majuscules pour plus de simplicite et pragmatisme.

L'objectif c'est de ne pas alourdir le binaire avec des features pas optimales pour des systemes à petites résources.


# Introduction de #![forbid(unsafe_code)] pour du safety.

## Utilisation

```toml
[dependencies]
embassy-ssd1306 = "0.2.2"
```

```rust
use embassy_ssd1306::Ssd1306;

let mut oled = Ssd1306::new(i2c, 0x3C);
oled.init().await.unwrap();

oled.draw_rect(0, 0, 128, 64, true);
oled.draw_i16(0, 0, -1234);
oled.flush().await.unwrap();
```

# Exemple Pico 2 et driver GY-Bmi160

````rust 

#![no_std]
#![no_main]
#![forbid(unsafe_code)]


use cortex_m_rt as _;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::i2c::{Config as I2cConfig, I2c, Async};
use embassy_time::{Duration, Timer, with_timeout};
use {panic_halt as _, embassy_rp as _};

use embassy_embedded_hal::shared_bus::asynch::i2c::I2cDevice;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::mutex::Mutex;
use static_cell::StaticCell;

use embassy_gy_bmi160::Bmi160;
use embassy_gy_bmi160::signals::{ACCEL_SIGNAL, GYRO_SIGNAL};

// ICI : On utilise la crate
use embassy_ssd1306::Ssd1306; 

use rp2350_linker as _;
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::I2C0;



bind_interrupts!(struct Irqs {
    I2C0_IRQ => embassy_rp::i2c::InterruptHandler<I2C0>;
});

static I2C_BUS: StaticCell<Mutex<NoopRawMutex, I2c<'static, I2C0, Async>>> = StaticCell::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(embassy_rp::config::Config::default());

    let mut i2c_config = I2cConfig::default();
    i2c_config.frequency = 400_000; // Fast Mode pour un flush fluide

    let i2c = I2c::new_async(p.I2C0, p.PIN_5, p.PIN_4, Irqs, i2c_config);
    let i2c_bus = Mutex::<NoopRawMutex, _>::new(i2c);
    let i2c_bus = I2C_BUS.init(i2c_bus);

    let oled_i2c = I2cDevice::new(i2c_bus);
    let bmi_i2c = I2cDevice::new(i2c_bus);

    let oled = Ssd1306::new(oled_i2c, 0x3C);
    let bmi = Bmi160::new(bmi_i2c, 0x68);

    spawner.spawn(system_task(oled, bmi)).unwrap();

    let mut led = Output::new(p.PIN_25, Level::Low);
    loop {
        led.toggle();
        Timer::after_millis(200).await;
    }
}

#[embassy_executor::task]
async fn system_task(
    mut oled: Ssd1306<I2cDevice<'static, NoopRawMutex, I2c<'static, I2C0, Async>>>,
    mut bmi: Bmi160<'static, I2cDevice<'static, NoopRawMutex, I2c<'static, I2C0, Async>>>,
) {
    let mut imu_ready = false;

    // 1. Init OLED 
        // Splash Screen JC-OS
        oled.draw_rect(0, 0, 128, 64, true);
        oled.draw_hline(10, 32, 108, true);
        let _ = oled.flush().await;
        Timer::after_millis(800).await;
        oled.clear();
    }

    //  2. Init IMU 
    if let Ok(Ok(_)) = with_timeout(Duration::from_millis(150), bmi.init()).await {
        imu_ready = true;
    } else {
        bmi.set_address(0x69);
        if let Ok(Ok(_)) = with_timeout(Duration::from_millis(150), bmi.init()).await {
            imu_ready = true;
        }
    }

    //  3. Boucle principale 
    loop {
        oled.clear();

        if imu_ready {
            // Section GYRO (Haut) 
            if let Ok(g) = bmi.read_gyro().await {
                GYRO_SIGNAL.signal(g);
                oled.draw_i16(0, 0, g.x); 
                oled.draw_i16(45, 0, g.y); 
                oled.draw_i16(90, 0, g.z);
            }

            // Ligne de séparation au milieu
            oled.draw_hline(0, 31, 128, true);

            // Section ACCEL (Bas) 
            if let Ok(a) = bmi.read_accel().await {
                ACCEL_SIGNAL.signal(a);
                oled.draw_i16(0, 5, a.x); 
                oled.draw_i16(45, 5, a.y); 
                oled.draw_i16(90, 5, a.z);
            }
        } else {
            // Si l'IMU est absent, on l'affiche proprement
            oled.draw_i16(30, 3, 404); // Erreur 404
        }

        // Mise à jour de l'écran physique
        let _ = oled.flush().await;
        
        Timer::after_millis(50).await; // Fréquence de rafraîchissement rapide
    }
}


````

# Exemple version 0.2.0 
## Exemple d'utilisation

```rust
use embassy_ssd1306::Ssd1306;

let mut oled = Ssd1306::new(i2c, 0x3C);
oled.init().await.unwrap();

// Texte
oled.draw_str(0, 0, b"HELLO WORLD");

// Nombre signé
oled.draw_str(0, 1, b"TEMP ");
oled.draw_i16(30, 1, -12);

// Rectangle de bordure
oled.draw_rect(0, 0, 128, 64, true);

oled.flush().await.unwrap();
```



## Fonctionnalités

- `draw_pixel` / `draw_hline` / `draw_vline`
- `draw_rect` / `draw_filled_rect`
- `draw_bitmap` (1bpp, MSB à gauche)
- `draw_char` / `draw_i16` / `draw_str` (font 5x7, chiffres, signe, lettres A–Z)
- Framebuffer 1024 bytes en RAM, flush optimisé page par page

## Licence

GPL-2.0-or-later — Copyright (C) 2026 Jorge Andre Castro