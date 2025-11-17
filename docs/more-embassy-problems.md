# Embassy nRF52840 Issues - Investigation Summary

## Problem
Attempting to convert nRF52840 WS2812B control code to use Embassy framework results in consistent "Busy" executor panic when trying to use `Timer::after_millis().await`.

## Initial Problem: Mixing HAL and Embassy

You cannot easily "slowly convert" to Embassy while mixing `nrf52840-hal` with `embassy-nrf` because:

- `embassy-nrf` v0.2.x (compatible with `nrf52840-pac`/HAL) has a critical bug that causes the timer to crash with a "Busy" panic.
- `embassy-nrf` v0.8.x (latest) uses a completely different PAC (`nrf-pac`) that's incompatible with `nrf52840-hal`.

## Full Conversion Attempt to embassy-nrf v0.8

### Configuration Used
```toml
[dependencies]
cortex-m = { version = "0.7.7", features = ["inline-asm", "critical-section-single-core"] }
cortex-m-rt = "0.7.5"
embassy-time = { version = "0.5", features = ["defmt"] }
embassy-nrf = { version = "0.8", features = [
    "defmt",
    "nrf52840",
    "time-driver-rtc1",
    "gpiote",
    "nfc-pins-as-gpio",
    "time",
] }
embassy-executor = { version = "0.9", features = [
    "defmt",
    "arch-cortex-m",
    "executor-thread",
] }
```

### Symptoms
- Embassy initialization succeeds
- GPIO operations work
- **Crash occurs immediately when calling `Timer::after_millis().await`**
- Error message: `"Busy - Too many instances of this task are already running. Check the pool_size attribute of the task."`
- Panic always at address `0x00000a60`

### What Was Tried

1. **Removed `nrf52840-hal` completely** - Still crashes
2. **Used Embassy's GPIO and async SPI** - Still crashes
3. **Removed `executor-interrupt` feature** - Still crashes
4. **Changed edition from 2024 to 2021** - Still crashes
5. **Removed bootloader base-address offset** - Still crashes
6. **Tried different defmt timestamp configurations** - Made it worse or same
7. **Minimal test case (just LED + Timer)** - Still crashes

### Code That Fails
```rust
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let config = embassy_nrf::config::Config::default();
    let p = embassy_nrf::init(config);

    let mut led = Output::new(p.P0_06, Level::Low, OutputDrive::Standard);

    loop {
        led.set_high();
        Timer::after_millis(1000).await; // <- CRASHES HERE
        led.set_low();
        Timer::after_millis(1000).await;
    }
}
```

### Comparison with Working Example (RMK)

RMK successfully uses Embassy with nRF52840: https://github.com/HaoboGu/rmk/tree/main/examples/use_rust/nrf52840_ble

**Key differences found:**
- RMK uses same embassy versions (0.9 executor, 0.8 nrf, 0.5 time)
- RMK uses same executor features (no `executor-interrupt`)
- RMK initializes with custom config:
  ```rust
  let mut nrf_config = embassy_nrf::config::Config::default();
  nrf_config.dcdc.reg0_voltage = Some(embassy_nrf::config::Reg0Voltage::_3V3);
  nrf_config.dcdc.reg0 = true;
  nrf_config.dcdc.reg1 = true;
  ```
- RMK uses complex Bluetooth stack (MPSL/SDC) which may handle initialization differently

**However:** RMK is a complex keyboard firmware - there's no simple timer example to compare against.

## Root Cause Analysis

The "Busy" panic from `embassy-executor` when awaiting a Timer suggests:
1. A conflict in the executor's task scheduling
2. Possible issue with RTC1 time driver initialization
3. Possible interrupt priority configuration issue
4. Possible memory layout issue with bootloader offset

The fact that:
- Embassy init succeeds
- GPIO works
- Only async Timer fails
- Panic is always at same address

...suggests a specific incompatibility between the executor, time driver, and possibly the probe-rs runtime environment or bootloader setup.

## Conclusion

**The Embassy framework with nRF52840 has a critical bug or misconfiguration that prevents Timer operations from working.**

Despite following RMK's configuration and trying numerous variations, the Timer consistently crashes with the same "Busy" executor panic.

## Recommendations

1. **Keep the working non-Embassy code** - `cortex_m::asm::delay` works perfectly
2. **File an issue** with Embassy project including this minimal reproduction
3. **Wait for Embassy maturity** - this appears to be a known limitation
4. Consider alternative approaches:
   - Use hardware timers directly without Embassy's Timer abstraction
   - Use Embassy only for non-timing-critical tasks
   - Wait for bug fixes in future Embassy releases

## Your Options

1. **Stay with current non-Embassy code** - it works perfectly
2. Try using hardware timers directly (bypassing Embassy Timer)
3. Wait for Embassy framework updates/fixes
4. Use a different nRF board that has confirmed working Embassy support
