#![no_std]

//! # jhd1313m1_i2c
//!
//! ` jhd1313m1_i2c` is a library for controlling the i2c JHD1313M1 display using Rust and the Embassy framework.
//!
//! This library provides an easy-to-use interface for interacting with the JHD1313M1 display, allowing
//! users to set colors, write messages, and control other aspects of the display.
//!
//! ## Usage Example
//!
//! ```rust
//!  use embassy_executor::Spawner;
//! use embassy_rp::{bind_interrupts, i2c};
//! use embassy_rp::i2c::{Config, InterruptHandler};
//! use embassy_rp::peripherals::I2C1;
//!
//!  // ...first create the binding for I2C IRQs ...
//! bind_interrupts!(struct Irqs {
//!     I2C1_IRQ => InterruptHandler<I2C1>;
//! });
//!
//! #[embassy_executor::main]
//!  async fn main(_spawner: Spawner) {
//!     // Initialise Peripherals
//!     let p = embassy_rp::init(Default::default());
//!     let  i2c = i2c::I2c::new_async(p.I2C1, p.PIN_7, p.PIN_6, Irqs, Config::default());
//!     let mut display = jhd1313m1_i2c::JHD1313M1::new(i2c);
//!     display.init().await;
//!     display.clear().await;
//!     display.write("Hello World").await;
//! }
//! ```
//!
//! For more details on each function, refer to the specific documentation for each method and structure.

pub mod display;
pub use display::JHD1313M1;