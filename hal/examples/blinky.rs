//! Blinks an LED

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use at32_hal as hal;

use crate::hal::{gpio::GpioExt, pac};
use cortex_m_rt::entry;

pub use defmt::{debug, error, info, trace, warn};
use defmt_rtt as _;

#[entry]
fn main() -> ! {
    info!("start");
    let p = pac::Peripherals::take().unwrap();

    info!("Init Led");
    let gpioc = p.gpioc.split();
    let mut led = gpioc.pc4.into_push_pull_output();

    loop {
        info!("Set Led High");
        for _ in 0..10_000_000 {
            led.set_high();
        }

        info!("Set Led low");
        for _ in 0..10_000_000 {
            led.set_low();
        }
    }
}

use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};

#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}
