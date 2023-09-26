/// Control module for the JHD1313M1 display.
///
/// This module provides functions to interact with the JHD1313M1 display via I2C.
use defmt::{debug, info};
use embassy_rp::i2c::{self, Async};
use embassy_time::Delay;
use embedded_hal_async::i2c::I2c;
use embedded_hal_async::delay::DelayUs;

// Constants representing various LCD and RGB addresses.
const LCD_ADDRESS: u8 = 0x3E;
const RGB_ADDRESS: u8 = 0x62;
const CMD_CLEAR: u8 = 0x01;
const CMD_HOME: u8 = 0x02;
const CMD_ENTRY: u8 = 0x06;
const CMD_DISPLAY_OFF: u8 = 0x08;
const CMD_DISPLAY_ON: u8 = 0x0C;
const CMD_FUNCTION_SET: u8 = 0x38;
const CMD_SET_DDRAM_ADDR: u8 = 0x80;

/// Represents the JHD1313M1 display with associated I2C functionality.
pub struct JHD1313M1<'a, I: i2c::Instance> {
    i2c: i2c::I2c<'a, I, Async>,
    display_control: u8,
    entry_display_mode: u8,
}

impl<'a, I: i2c::Instance> JHD1313M1<'a, I> {
    /// Creates a new instance of the JHD1313M1 display.
    pub fn new(i2c: i2c::I2c<'a, I, Async>) -> Self {
        info!("Creating i2c");
        JHD1313M1 {
            i2c,
            display_control: 0,
            entry_display_mode: 0,
        }
    }

    /// Initializes the display.
    pub async fn init(&mut self) {
        info!("init display");
        Delay.delay_ms(50).await;
        self.command(CMD_FUNCTION_SET).await;
        Delay.delay_ms(5).await;
        self.command(CMD_FUNCTION_SET).await;
        Delay.delay_ms(1).await;
        self.command(CMD_FUNCTION_SET).await;
        self.command(CMD_DISPLAY_ON).await;
        self.command(CMD_CLEAR).await;
        self.command(CMD_ENTRY).await;
    }

    /// Sets the RGB color of the backlight.
    pub async fn set_color(&mut self, r: u8, g: u8, b: u8) {
        let mut buf = [0u8; 2];
        buf[0] = 0x04;
        buf[1] = r;
        self.i2c.write(RGB_ADDRESS, &buf).await.unwrap();
        buf[0] = 0x03;
        buf[1] = g;
        self.i2c.write(RGB_ADDRESS, &buf).await.unwrap();
        buf[0] = 0x02;
        buf[1] = b;
        self.i2c.write(RGB_ADDRESS, &buf).await.unwrap();
    }

    /// Writes a string message to the display.
    pub async fn write(&mut self, msg: &str) {
        for ch in msg.chars() {
            self.data(ch as u8).await;
        }
    }

    /// Sets the cursor position on the display.
    pub async fn set_cursor(&mut self, row: u8, col: u8) {
        let addr = CMD_SET_DDRAM_ADDR + (0x40 * row) + col;
        self.command(addr).await;
    }

    /// Clears the display.
    pub async fn clear(&mut self) {
        self.command(CMD_CLEAR).await;
        Delay.delay_ms(2).await;
    }

    /// Returns the cursor to the home position.
    pub async fn home(&mut self) {
        self.command(CMD_HOME).await;
        Delay.delay_ms(2).await;
    }

    /// Sends a command to the display.
    async fn command(&mut self, cmd: u8) {
        let buf = [0x80, cmd];
        debug!("Sending command: {=u8}", cmd);
        self.i2c.write(LCD_ADDRESS, &buf).await.unwrap();
    }

    /// Sends data to the display.
    async fn data(&mut self, data: u8) {
        let buf = [0x40, data];
        debug!("Writing data: {=u8}", data);
        self.i2c.write(LCD_ADDRESS, &buf).await.unwrap();
    }

    /// Creates a custom character on the display.
    pub async fn create_char(&mut self, slot: u8, data: &[u8; 8]) {
        let addr = 0x40 + (slot << 3);
        self.command(addr).await;
        for i in 0..8 {
            self.data(data[i]).await;
        }
    }

    /// Toggles the cursor visibility on the display.
    pub async fn cursor_on(&mut self, on: bool) {
        if on {
            self.display_control |= 0x02;
        } else {
            self.display_control &= !0x02;
        }
        self.command(0x08 | self.display_control).await;
    }

    /// Toggles the cursor blink on the display.
    pub async fn cursor_blink_on(&mut self, on: bool) {
        if on {
            self.display_control |= 0x01;
        } else {
            self.display_control &= !0x01;
        }
        self.command(0x08 | self.display_control).await;
    }

    /// Toggles the backlight of the display.
    pub async fn backlight_on(&mut self, on: bool) {
        if on {
            self.set_color(255, 255, 255).await;
        } else {
            self.set_color(0, 0, 0).await;
        }
    }

    /// Scrolls the display content to the left.
    pub async fn scroll_display_left(&mut self) {
        self.command(0x18).await;
    }

    /// Scrolls the display content to the right.
    pub async fn scroll_display_right(&mut self) {
        self.command(0x1E).await;
    }

    /// Sets the text entry mode from left to right.
    pub async fn entry_left_to_right(&mut self, on: bool) {
        if on {
            self.entry_display_mode |= 0x02;
        } else {
            self.entry_display_mode &= !0x02;
        }
        self.command(0x04 | self.entry_display_mode).await;
    }

    /// Toggles the autoscroll feature of the display.
    pub async fn autoscroll_on(&mut self, on: bool) {
        if on {
            self.entry_display_mode |= 0x01;
        } else {
            self.entry_display_mode &= !0x01;
        }
        self.command(0x04 | self.entry_display_mode).await;
    }

    /// Powers on the display.
    pub async fn power_on(&mut self) {
        self.command(CMD_DISPLAY_ON).await;
    }

    /// Powers off the display.
    pub async fn power_off(&mut self) {
        self.command(CMD_DISPLAY_OFF).await;
    }
}
