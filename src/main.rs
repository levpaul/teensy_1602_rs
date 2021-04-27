//! The starter code slowly blinks the LED, and sets up
//! USB logging.

#![no_std]
#![no_main]
#![allow(unused_must_use)]
#![feature(alloc_error_handler)]
#![feature(never_type)]

use alloc_cortex_m::CortexMHeap;
#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();
use core::alloc::Layout;

extern crate alloc;
use alloc::boxed::Box;

use teensy4_bsp as bsp;
use teensy4_bsp::hal::gpio;
use teensy4_panic as _;

mod liquid_crystal;
mod logging;

const LED_PERIOD_MS: u32 = 1_000;

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = bsp::Peripherals::take().unwrap();
    let mut systick = bsp::SysTick::new(cortex_m::Peripherals::take().unwrap().SYST);
    let pins = bsp::t40::into_pins(p.iomuxc);
    let mut led = bsp::configure_led(pins.p13);

    // See the `logging` module docs for more info.
    assert!(logging::init().is_ok());
    // Init
    let mut rs = gpio::GPIO::new(pins.p12).output();
    let mut en = gpio::GPIO::new(pins.p11).output();
    let mut d4 = gpio::GPIO::new(pins.p5).output();
    let mut d5 = gpio::GPIO::new(pins.p4).output();
    let mut d6 = gpio::GPIO::new(pins.p3).output();
    let mut d7 = gpio::GPIO::new(pins.p2).output();

    let mut lcd = liquid_crystal::LCD {
        en: Box::new(en),
        rs: Box::new(rs),
        d4: Box::new(d4),
        d5: Box::new(d5),
        d6: Box::new(d6),
        d7: Box::new(d7),
        st: &mut systick,
    };

    lcd.init();
    let mut i = 20;
    loop {
        i = i + 1 % 255;
        led.toggle();
        lcd.delay(LED_PERIOD_MS);
        lcd.write_char(i);
        log::info!("Hello world");
    }
}

#[alloc_error_handler]
fn oom(_: Layout) -> ! {
    loop {}
}
