
This is a direct port of [Marek Miettinen's](https://github.com/marikka) 
[micro-tetris](http://marekmiettinen.fi/micropong/index.html) from Cortex-M0 to Cortex-M4. Specifically from the STM32F042K6 (NUCLEO-F042K6) to the STM32F103 (eg. [Blue-Pill](https://github.com/WeActTC/BluePill-Plus))

It's a fun showcase of the [SSD1306 OLED display](https://crates.io/crates/ssd1306).

The original is in [this branch](https://github.com/marikka/micropong/tree/tetris). I didn't (yet?) port the pong game [here](https://github.com/marikka/micropong).

### Flashing

`$ rustup target add thumbv7m-none-eabi`

`$ cargo install cargo-flash`

`$ cargo flash --release --chip STM32F103C8`

### License

This project is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
