# nrf-mantra-demo

This repository contains a base Rust template to run tests on a [nRF52840 DK](https://www.nordicsemi.com/Products/Development-hardware/nRF52840-DK)
using OpenOCD as connection.

**Note:** Tests must be written using the [`defmt-test`](https://crates.io/crates/defmt-test) crate if you want to get a requirements traceability coverage that is understood by [mantra](https://crates.io/crates/mantra).

## Prerequisites

**Install:**
- `cargo install mantra` (requires a C compiler available on Path)
- `cargo install embedded-runner`
- `cargo install flip-link`
- `cargo install just`
- Ensure `arm-none-eabi-gdb` is available on Path

Connect a nRF52840 DK.

## Usage

To execute tests, run:

```
just test
```

This will connect to the nRF52840, flash the test binaries, run the tests, and collect requirements traceability coverage in a `coverage.json` file.

To collect requirements traceability information, run:

```
just mantra
```

This will generate a `mantra_report.html` traceability report.

## Internals

The OpenOCD configuration used is defined at `.embedded/openocd.cfg`, and `embedded-runner` is set as Cargo runner in `.cargo/config.toml`.
`embedded-runner` will pass a generated GDB script to `arm-none-eabi-gdb` that uses OpenOCD as connection and RTT to retrieve defmt logs.
On the host side, the defmt logs are then decoded and shown in the terminal. If `defmt-test` and `mantra` logs are detected, `embedded-runner` will extract them into a coverage file per test binary. Multiple coverage files are then converted using `embedded-runner collect`.
