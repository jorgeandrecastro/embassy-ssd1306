//! Opcodes du contrôleur SSD1306 et octets de contrôle I2C.

/// Octet de contrôle : le(s) octet(s) suivant(s) sont des commandes.
pub const CONTROL_CMD: u8 = 0x00;
/// Octet de contrôle : le(s) octet(s) suivant(s) sont des données GDDRAM.
pub const CONTROL_DATA: u8 = 0x40;

pub const DISPLAY_OFF: u8 = 0xAE;
pub const DISPLAY_ON: u8 = 0xAF;
pub const SET_OSC_FREQ: u8 = 0xD5;
pub const SET_MULTIPLEX: u8 = 0xA8;
pub const SET_DISPLAY_OFFSET: u8 = 0xD3;
pub const SET_START_LINE: u8 = 0x40;
pub const CHARGE_PUMP: u8 = 0x8D;
pub const SET_MEMORY_MODE: u8 = 0x20;
pub const SEGMENT_REMAP: u8 = 0xA1;
pub const COM_SCAN_DEC: u8 = 0xC8;
pub const SET_COM_PINS: u8 = 0xDA;
pub const SET_CONTRAST: u8 = 0x81;
pub const SET_PRECHARGE: u8 = 0xD9;
pub const SET_VCOMH: u8 = 0xDB;
pub const RESUME_FROM_RAM: u8 = 0xA4;
pub const NORMAL_DISPLAY: u8 = 0xA6;
pub const SET_COLUMN_ADDR: u8 = 0x21;
pub const SET_PAGE_ADDR: u8 = 0x22;