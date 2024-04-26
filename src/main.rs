#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::{
    gpio::{Level, Output},
    pwm::{Config, Pwm},
};
use embassy_time::{Duration, Ticker, Timer};
use {defmt_rtt as _, panic_probe as _};

#[cortex_m_rt::pre_init]
unsafe fn before_main() {
    embassy_rp::pac::SIO.spinlock(31).write_value(1);
}

#[derive(PartialEq, Eq)]
enum Toggle {
    On,
    Off,
}

impl Toggle {
    fn next(&self) -> Self {
        match self {
            Toggle::On => Toggle::Off,
            Toggle::Off => Toggle::On,
        }
    }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let mut c: Config = Default::default();
    c.top = u16::MAX;
    c.compare_a = 0;
    c.compare_b = 0;

    let mut out = Output::new(p.PIN_16, Level::Low);
    let mut out2 = Output::new(p.PIN_17, Level::Low);

    let mut ticker = Ticker::every(Duration::from_micros(20));

    let mut toggle = Toggle::Off;

    loop {
        if toggle == Toggle::On {
            out.set_high();
            out2.set_low();
        }
        ticker.next().await;
        out.set_low();
        out2.set_high();
        ticker.next().await;
        toggle = toggle.next();
    }
}
