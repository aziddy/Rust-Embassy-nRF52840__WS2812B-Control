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