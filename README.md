# `at32-hal`

🚧 *Work in progress*

_at32-hal_ contains a multi device hardware abstraction on top of the
peripheral access API for the Artery AT32 series microcontrollers. The
selection of the MCU is done by feature gates, typically specified by board
support crates.

## Usage

This crate will eventually contain support for multiple microcontrollers in the
at32 family. Which specific microcontroller you want to build for has to be
specified with a feature, for example `at32f435`.

Currently supported configurations are:

* at32f413
* at32f415
* at32f421
* at32f435
* at32f437

### Building an Example

If you are compiling the crate on its own for development or running examples,
specify your microcontroller on the command line. For example:

```
cargo build --example blinky --features at32f435,defmt
```

## Running examples

Examples can be built and run using `cargo run`. It is necessary to provide any
required features followed by the name of the chip.

```
cargo run --example blinky --features at32f435,defmt --release -- --chip AT32F435CGU7
```

A list of chips supported by probe-rs can be found by running

```
probe-rs chip list
```

For furher information, see the documentation for [probe-rs](https://probe.rs/).

### Using as a Dependency

When using this crate as a dependency in your project, the microcontroller can
be specified as part of the `Cargo.toml` definition.

```
[dependencies]
at32-hal = { version = "0.0.1", features = ["at32f435", "defmt"] }
```

## Documentation

The documentation can be found at [docs.rs](https://docs.rs/at32-hal/).

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
