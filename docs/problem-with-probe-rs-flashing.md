# Problem with probe-rs Flashing

## Issue

Firmware works when uploaded via UF2 file but fails when flashed with `cargo run --release` (which uses probe-rs).

## Root Cause

The nRF52840 board uses the Adafruit bootloader, which occupies the first section of flash memory up to address `0x00026000`. Our firmware is configured in [memory.x](../memory.x) to start at `0x00026000` to avoid overwriting the bootloader.

However, probe-rs by default flashes firmware starting at address `0x00000000`, which:
1. Doesn't match where the linker expects the code to be
2. May overwrite parts of the bootloader

When using UF2 upload through the bootloader, the bootloader correctly places the firmware at the right address.

## Solution

Configure probe-rs to flash at the correct base address by adding the `--base-address` flag in [.cargo/config.toml](../.cargo/config.toml):

```toml
[target.thumbv7em-none-eabihf]
runner = "probe-rs run --chip nRF52840_xxAA --base-address 0x00026000"
```

This tells probe-rs to flash the firmware at `0x00026000`, matching the memory layout defined in memory.x.

## Memory Layout

```
0x00000000 - 0x00025FFF: Adafruit Bootloader (152K)
0x00026000 - 0x000FFFFF: Application Firmware (848K)
```
