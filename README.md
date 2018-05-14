# rust-lpc82x [![crates.io](https://img.shields.io/crates/v/lpc82x.svg)](https://crates.io/crates/lpc82x) [![Documentation](https://docs.rs/lpc82x/badge.svg)](https://docs.rs/lpc82x) [![Build Status](https://travis-ci.org/braun-robotics/rust-lpc82x.svg?branch=master)](https://travis-ci.org/braun-robotics/rust-lpc82x)

## Introduction

Low-level register mappings for the [NXP LPC82x] family of ARM Cortex-M0+ microcontrollers, written in [Rust]. The code is generated automatically from the [SVD file] available from ARM, using [svd2rust].

The purpose of this crate is to give embedded programs or libraries written Rust access to the complete functionality of LPC82x MCUs.

Please also check out [LPC82x HAL], a higher-level crate for interfacing with the LPC82x.


## Usage

Add this to the `[dependencies]` section of your `Cargo.toml` to include rust-lpc82x in your Cargo project:

``` toml
lpc82x = "0.4"
```

This crate includes an optional `rt` feature that can be activated by adding this instead:

``` toml
lpc82x = { version = "0.4", features = ["rt"] }
```

The `rt` feature includes the [cortex-m-rt] crate and provides overridable interrupt handlers. Please refer to the [svd2rust documentation] for further details.

## Example

Here's a simple example that could be extended into an embedded program that blinks an LED.

``` rust
extern crate lpc82x;

// This bit represents the PIO0_3 pin. We could connect an LED there.
const PIN: u32 = 0x1 << 3;

// Get a reference to the GPIO_PORT peripheral
let gpio = lpc82x::GPIO_PORT.get();

unsafe {
    // Set pin direction to "output"
    (*gpio).dir0.modify(|r, w| w.dirp().bits(r.dirp().bits() | PIN));

    loop {
        // Set pin to HIGH
        (*gpio).set0.modify(|r, w| w.setp().bits(r.setp().bits() | PIN));

        // Sleep here for some time

        // Set pin to LOW
        (*gpio).clr0.write(|w| w.clrp().bits(PIN));

        // Sleep here for some time
    }
}
```

## Documentation

For specific information on the API, check out the **[API reference]**.

All code in this crate is automatically generated by [svd2rust], so check out the **[svd2rust documentation]** for more general information about how the API works.

In addition, the [LPC82x user manual] contains extensive documentation on how to work with the microcontroller.

## Status

This crate is complete and actively maintained, but not all parts of it have been tested. The experience so far has shown that the original SVD file has quite a few problems. It's likely there are still undetected bugs. Please [open an issue], if you find any problems. Known issues are tracked [on GitHub][list of open issues].

Another problem that we inherit from the SVD file is that some register and field names are very weird. Those seem to be generated from human-readable documentation, meaning they sometimes read like cut-off sentences.

At this point, there is no guarantee of API stability. This means that we reserve the right to make changes to the API, that might break existing programs when they upgrade.

You need a nightly version of Rust to use this crate. If you installed Rust via [rustup], you can switch to the nightly version with `rustup default nightly`.

## Update Procedure

The repository contains an [update script], that can be used to re-generate the source code. This script updates all required tools ([svd2rust] and [rustfmt]), downloads the [SVD file], applies various patches to it, and then re-generates the code.

The patches that are applied to the SVD file are relatively minimal, and are just intended to fix various problems with the file that otherwise would prevent code generation, or would lead to incorrect code being generated.


## License

This project is open source software, licensed under the terms of the [Zero Clause BSD License][Zero Clause BSD License] (0BSD, for short). This basically means you can do anything with the software, without any restrictions, but you can't hold the authors liable for problems.

See [LICENSE] for full details.


**Supported by [Braun Robotics]**


[Rust]: https://www.rust-lang.org/
[NXP LPC82x]: http://www.nxp.com/products/microcontrollers-and-processors/arm-based-processors-and-mcus/lpc-cortex-m-mcus/lpc800-series-cortex-m0-plus-mcus/low-cost-microcontrollers-mcus-based-on-arm-cortex-m0-plus-cores:LPC82X
[SVD file]: http://ds.arm.com/media/resources/db/chip/nxp/lpc824m201jdh20/LPC82x.svd
[svd2rust]: https://crates.io/crates/svd2rust
[LPC82x HAL]: https://crates.io/crates/lpc82x-hal
[cortex-m-rt]: https://crates.io/crates/cortex-m-rt
[svd2rust documentation]: https://docs.rs/svd2rust/0.11.4/svd2rust/
[API reference]: https://docs.rs/lpc82x
[LPC82x user manual]: https://www.nxp.com/docs/en/user-guide/UM10800.pdf
[open an issue]: https://github.com/braun-robotics/rust-lpc82x/issues/new
[list of open issues]: https://github.com/braun-robotics/rust-lpc82x/issues
[rustup]: https://rustup.rs/
[update script]: https://github.com/braun-robotics/rust-lpc82x/blob/master/scripts/update
[rustfmt]: https://crates.io/crates/rustfmt
[Zero Clause BSD License]: https://opensource.org/licenses/FPL-1.0.0
[LICENSE]: https://github.com/braun-robotics/rust-lpc82x/blob/master/LICENSE
[Braun Robotics]: https://braun-robotics.com/
