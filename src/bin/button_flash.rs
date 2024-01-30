#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use core::sync::atomic::{AtomicBool, Ordering};

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::exti::ExtiInput;
use embassy_stm32::gpio::{AnyPin, Input, Output, Pull};
use embassy_stm32::gpio::{Level, Speed};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;
use embassy_time::{Instant, Timer};
use {defmt_rtt as _, panic_probe as _};

/// `true` when the key is being pressed
static PRESSED: AtomicBool = AtomicBool::new(false);

/// Signals the thread that the button has been pressed
static PRESSED_START: Signal<CriticalSectionRawMutex, ()> = Signal::new();

#[embassy_executor::task]
async fn flash(pin: AnyPin) {
    let mut led = Output::new(pin, Level::High, Speed::Low);
    loop {
        PRESSED_START.wait().await; // Waiting to check if the button has been pressed
        while PRESSED.load(Ordering::SeqCst) {
            // Checks how long the button has been pressed
            led.set_low();
            Timer::after_millis(50).await;
            led.set_high();
            Timer::after_millis(50).await;
        }
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    let button = Input::new(p.PA0, Pull::Up);
    let mut button = ExtiInput::new(button, p.EXTI0);
    spawner.spawn(flash(p.PC13.into())).unwrap();

    // Main loop toggles shared bool
    loop {
        button.wait_for_falling_edge().await;
        let start = Instant::now();
        PRESSED.store(true, Ordering::SeqCst);
        PRESSED_START.signal(());
        info!("Pressed!");

        button.wait_for_rising_edge().await;
        PRESSED.store(false, Ordering::SeqCst);
        let elapsed = start.elapsed().as_micros() as f64 / 1_000_000.0;
        info!("Released after {} seconds!", elapsed);
    }
}
