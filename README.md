
# JHD1313M1 Library for Rust

A library to control the i2c JHD1313M1 display using Rust, based on the Rust Embassy framework.

## Features

- **Initialization**: Set up the display with essential configurations.
- **RGB Backlight Control**: Adjust the RGB backlight color of the display (if it has RGB backlight)
- **Text Writing**: Write text messages directly to the display.
- **Cursor Control**:
  - Position the cursor at specific rows and columns.
  - Control cursor visibility and blinking.
- **Display Control**:
  - Clear the display content.
  - Return the cursor to the home position.
  - Turn the display on or off.
- **Character Customization**: Create custom characters for the display.
- **Backlight Control**: Turn the backlight on or off with a simple function.
- **Scrolling**:
  - Scroll the display content to the left or right.
  - Control the direction of text entry, from left to right.
  - Enable or disable autoscroll.

## Dependencies

- **Debugging & Logging**:
  - `defmt`

- **Embassy Framework**:
  - `embassy-rp`
  - `embassy-time`

- **Embedded Utilities**:
  - `embedded-hal-async`

## Installation

To add this library to your Rust project, add the following line to your `Cargo.toml` file:

```toml
[dependencies]
jhd1313m1_i2c = { version = "0.0.1", git = "https://github.com/chrnueve/jhd1313m1-i2c"}
```

Then, run:

```bash
$ cargo build
```

## Basic Usage

```rust
#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(async_fn_in_trait)]

use {defmt_rtt as _, panic_probe as _};
use embassy_executor::Spawner;
use embassy_rp::{bind_interrupts, i2c};
use embassy_rp::i2c::{Config, InterruptHandler};
use embassy_rp::peripherals::I2C1;

bind_interrupts!(struct Irqs {
    I2C1_IRQ => InterruptHandler<I2C1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Initialise Peripherals
    let p = embassy_rp::init(Default::default());

    let  i2c = i2c::I2c::new_async(p.I2C1, p.PIN_7, p.PIN_6, Irqs, Config::default());
    let mut display = jhd1313m1-i2c::JHD1313M1::new(i2c);

    display.init().await;
  
    display.clear().await;
    display.write("Hello World").await;
}
```

## Contributing

Contributions are welcome! Please read the [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct, and the process for submitting pull requests.

## Code of Conduct

Please read our [Code of Conduct](CODE_OF_CONDUCT.md) to maintain an inclusive and respectful community.

## License

This project is licensed under the GNU General Public License v3.0 - see the [LICENSE.md](LICENSE.md) file for details.

## A Note to the Community

Hello everyone,

I'd like to take a moment to apologize in advance for any mistakes or unconventional practices you might find in this repository. As a seasoned Java and Python developer, I'm currently on a journey to learn Rust. This repository represents one of my initial challenges in this new language.

I genuinely appreciate any contributions that can help me improve and get better with Rust. Your feedback, suggestions, and pull requests are more than welcome. Let's learn and grow together!

Thank you for your understanding and support.

Best regards,
Christian
