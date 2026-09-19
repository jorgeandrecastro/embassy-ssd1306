//! Driver I2C : initialisation du contrôleur et transfert du framebuffer.

use core::ops::{Deref, DerefMut};

use embassy_time::Timer;
use embedded_hal_async::i2c::I2c;

use crate::command::*;
use crate::framebuffer::FrameBuffer;
use crate::{PAGES, SCREEN_WIDTH};

/// Instance principale du driver SSD1306.
///
/// Se déréférence vers [`FrameBuffer`] : toutes les méthodes `draw_*`
/// et `clear`/`fill` sont directement appelables sur `Ssd1306`.
pub struct Ssd1306<I: I2c> {
    i2c: I,
    /// Adresse I2C configurée (0x3C ou 0x3D).
    pub addr: u8,
    fb: FrameBuffer,
}

impl<I: I2c> Deref for Ssd1306<I> {
    type Target = FrameBuffer;
    fn deref(&self) -> &FrameBuffer {
        &self.fb
    }
}

impl<I: I2c> DerefMut for Ssd1306<I> {
    fn deref_mut(&mut self) -> &mut FrameBuffer {
        &mut self.fb
    }
}

impl<I: I2c> Ssd1306<I> {
    /// Crée une nouvelle instance du driver.
    ///
    /// * `i2c`  : bus I2C (ou `I2cDevice` partagé).
    /// * `addr` : adresse du composant (généralement 0x3C).
    pub fn new(i2c: I, addr: u8) -> Self {
        Self { i2c, addr, fb: FrameBuffer::new() }
    }

    /// Rend le bus I2C (utile pour le réutiliser ailleurs).
    pub fn release(self) -> I {
        self.i2c
    }

    // ── Commandes bas niveau ────────────────────────────────────────────

    async fn cmd(&mut self, c: u8) -> Result<(), I::Error> {
        self.i2c.write(self.addr, &[CONTROL_CMD, c]).await
    }

    async fn cmd2(&mut self, c: u8, d: u8) -> Result<(), I::Error> {
        self.i2c.write(self.addr, &[CONTROL_CMD, c, d]).await
    }

    async fn cmd3(&mut self, c: u8, d1: u8, d2: u8) -> Result<(), I::Error> {
        self.i2c.write(self.addr, &[CONTROL_CMD, c, d1, d2]).await
    }

    // ── Init ────────────────────────────────────────────────────────────

    /// Configure le SSD1306 et efface l'écran.
    ///
    /// Doit être appelé une fois avant toute opération d'affichage.
    pub async fn init(&mut self) -> Result<(), I::Error> {
        Timer::after_millis(200).await;

        self.cmd(DISPLAY_OFF).await?;
        self.cmd2(SET_OSC_FREQ, 0x80).await?;
        self.cmd2(SET_MULTIPLEX, 0x3F).await?; // 64 lignes
        self.cmd2(SET_DISPLAY_OFFSET, 0x00).await?;
        self.cmd(SET_START_LINE).await?;
        self.cmd2(CHARGE_PUMP, 0x14).await?; // pompe de charge ON
        self.cmd2(SET_MEMORY_MODE, 0x00).await?; // adressage horizontal
        self.cmd(SEGMENT_REMAP).await?;
        self.cmd(COM_SCAN_DEC).await?;
        self.cmd2(SET_COM_PINS, 0x12).await?;
        self.cmd2(SET_CONTRAST, 0xCF).await?;
        self.cmd2(SET_PRECHARGE, 0xF1).await?;
        self.cmd2(SET_VCOMH, 0x40).await?;
        self.cmd(RESUME_FROM_RAM).await?;
        self.cmd(NORMAL_DISPLAY).await?;
        self.cmd(DISPLAY_ON).await?;

        self.fb.clear();
        self.flush().await
    }

    // ── Flush ───────────────────────────────────────────────────────────

    /// Envoie le framebuffer complet vers l'écran, page par page.
    pub async fn flush(&mut self) -> Result<(), I::Error> {
        self.cmd3(SET_COLUMN_ADDR, 0, (SCREEN_WIDTH - 1) as u8).await?;
        self.cmd3(SET_PAGE_ADDR, 0, (PAGES - 1) as u8).await?;

        let mut buf = [0u8; SCREEN_WIDTH + 1];
        buf[0] = CONTROL_DATA;
        for page in 0..PAGES {
            let start = page * SCREEN_WIDTH;
            buf[1..].copy_from_slice(&self.fb.as_bytes()[start..start + SCREEN_WIDTH]);
            self.i2c.write(self.addr, &buf).await?;
        }
        Ok(())
    }
}