### Install target & tools:
rustup target add thumbv7em-none-eabihf
cargo install probe-rs --features cli

cargo install --force cargo-make






## Build & Flash:

### Build & Generate UF2 File (Adafruit Bootloader):
```
# Build
cargo build --release

# Convert to UF2
cargo make uf2 --release
```



### With SWD:
1) Flashes & log attached to the console:
```
cargo run --release
```

2) Only Flash to Chip
```
probe-rs download --chip nRF52840_xxAA --base-address 0x26000 target/thumbv7em-none-eabihf/release/nrf-blink
```

#### WARNING CARGO RUN/PROBE-RS FLASHING:
When using probe-rs to flash the firmware, it will flash the firmware at address `0x00000000`, which:
1. Doesn't match where the linker expects the code to be
2. May overwrite parts of the bootloader

When using UF2 upload through the bootloader, the bootloader correctly places the firmware at the right address.

So, when using probe-rs to flash the firmware, you need to use the `--base-address` flag to tell it to flash the firmware at the correct address.

```
probe-rs run --chip nRF52840_xxAA --base-address 0x00026000 target/thumbv7em-none-eabihf/release/<NAME_OF_BINARY>
```
