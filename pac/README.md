# PACs for AT32 microcontrollers

This repository contains Peripheral Access Crates (PACs) for AT32 series Cortex-M microcontrollers.

All these crates are automatically generated using [svd2rust].

For a more user-friendly interface to the peripherals, the [`at32-hal`] crates might be more appropriate.

Please refer to the [changelog] to see what changed in the last releases.

[changelog]: ./CHANGELOG.md
[`at32-hal`]: https://github.com/danclive/at32-hal
[svd2rust]: https://github.com/rust-embedded/svd2rust

## Supported Devices

* at32f413
* at32f415
* at32f421
* at32f435
* at32f437

## Usage

To use, add line to your Cargo.toml:

```toml
at32-pac = { version = "0.0.1", features = ["at32f435"] }
```
