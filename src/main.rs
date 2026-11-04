// Rust has three included modules: core, std, and alloc.
// std deals with eternal things like io, filesystems, etc,
// so we can't use it here for embedded programming. That sort of
// functionality is provided by crates specific to our embedded setup.
//
// We still have access to core, which includes many common types and functions.
// We can use alloc (for heap access) in no_std if we provide an allocator,
// though for flight software we may want to define all of our memory statically.
#![no_std]
// In embedded, our entry point is specified by macros like
// #[embassy_executor::main] or #[cortex_m_rt::entry]
#![no_main]

// imports
use defmt::*;
// `as _` means we are importing it without giving it a name.
// We just need the crate linked in.
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::Timer;
use panic_probe as _;

// Our entry point. #[embassy_executor::main] implicitly creates an executor, which
// is essentially an event loop we can spawn concurrent tasks from.
// using #[cortex_m_rt::entry] would alternatively skip the implicit executor,
// and we would create our own manually.
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // If two separate parts of the code owned the same peripheral, that
    // could cause all sorts of race conditions, so instead each peripheral
    // is a singleton on this struct, and rust's compiler keeps track of
    // the ownership of this peripheral.
    // If we wish to share access to a peripheral, that is then done through
    // interrupt-safe structures, like mutexes.
    //
    // init also sets up clocks. Config::default() runs the chip on its internal
    // oscillator at reset speed, not the full 480 MHz.
    let p = embassy_stm32::init(Default::default());

    // macro from defmt for logging over a debug connection
    info!("Hello World!");

    // pass in a peripheral from p, transferring its ownership to this mutable Output instance.
    // p.PB14 should be the red led on the Nucleo-H753ZI board
    let mut led = Output::new(p.PB14, Level::Low, Speed::Low);

    // self explanatory, loops forever
    loop {
        info!("led on!");
        led.set_high();
        // .await temporarily pauses the current task until the timer fires, 
        // giving the executor a chance to pass CPU time to another task,
        // or seeping if theres nothing else to do at the moment.
        // This is how we get concurrency.
        Timer::after_millis(500).await;

        info!("led off!");
        led.set_low();
        Timer::after_millis(500).await;
    }
}
