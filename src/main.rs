#![no_std]
#![no_main]

// Needed for the get_ccount function
#![feature(asm_experimental_arch)]

use core::arch::asm;
use core::fmt::Write;

use esp8266_hal::rng::Rng;
use esp8266_hal::uart::UART0Serial;
use esp8266_hal::prelude::*;
use esp8266_hal::target::{Peripherals, RNG, UART0};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = dp.GPIO.split();
    let mut led = pins.gpio2.into_push_pull_output();
    let (mut timer1, _) = dp.TIMER.timers();

    led.set_high().unwrap();

    for _ in 1..10 {
        timer1.delay_ms(500);
        led.toggle().unwrap();
    }

    // Setting up uart so it transmits on the RX and TX pins
    let mut uart = UART0::serial(dp.UART0, pins.gpio1, pins.gpio3);
    write!(&mut uart, "Hello World!\n\r");

    let mut rng = RNG::rng(dp.RNG);
    let mut buffer = [0u8; 8];
    rng.read(&mut buffer).unwrap();

    loop {}
}

/// https://0x04.net/~mwk/doc/xtensa.pdf#G10.453845
/// 
/// ccount is a register that gets incremented by 1 every clock cycle
/// 
/// It's 32 bits so it resets itself after some time by overflowing
/// 
/// If you know the cpu frequency you can use this to measure time
// We always inline this so we don't have the function call overhead and have more accurate measurements
#[inline(always)]
fn get_ccount() -> u32 {
    let ccount;

    unsafe { asm!("rsr {}, ccount", out(reg) ccount); }

    ccount
}
