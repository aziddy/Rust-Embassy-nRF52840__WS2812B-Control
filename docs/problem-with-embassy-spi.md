## Embassy SPI Problem

The problem was mixing async Embassy APIs with synchronous (blocking) code execution. Here's what was happening:

1. Initial code used embassy-executor with #[embassy_executor::main] - this creates an async runtime
2. Embassy's SPI driver (Spim::new()) requires interrupt handlers via bind_interrupts! and is designed for async operation
3. The "Multiple" error occurred because:
- Embassy was trying to initialize its async runtime
- But something in the initialization was failing (likely the executor spawner being "Busy")
- The defmt logging conflict made debugging harder

**The solution:**
- Switched from embassy-nrf (async HAL) to nrf52840-hal (blocking HAL)
- Changed from #[embassy_executor::main] to #[cortex_m_rt::entry] (simple entry point)
- Used blocking SPI: Spim::new() from nrf52840-hal works synchronously without interrupts
- No async/await, no executor, no interrupt bindings needed

TL;DR: Embassy is great for async embedded, but if you just want simple blocking code, the nrf52840-hal crate is simpler and works without an executor. Now you have SPIM0 (SPI0) working with WS2812 on P0.26! 