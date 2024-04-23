#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::{
    clocks::clk_sys_freq,
    pwm::{Config, Pwm},
};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[cortex_m_rt::pre_init]
unsafe fn before_main() {
    embassy_rp::pac::SIO.spinlock(31).write_value(1);
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let mut c: Config = Default::default();
    c.top = u16::MAX;
    let mut counter = 0u16;

    let mut pwm = Pwm::new_output_ab(p.PWM_SLICE0, p.PIN_16, p.PIN_17, c.clone());
    info!("clk_sys_freq(): {}", clk_sys_freq());
    let freq = (clk_sys_freq() as f64 * 0.25) as u32;
    info!("PWM frequency: {}", freq);
    enum Bounce {
        Forward,
        Backward,
    }

    let mut bounce = Bounce::Forward;

    loop {
        Timer::after_micros(3).await;
        if counter >= 100 {
            bounce = Bounce::Backward;
        }
        if counter <= 0 {
            bounce = Bounce::Forward;
        }
        match bounce {
            Bounce::Forward => {
                counter += 1;
                pwm.set_duty_a(counter).unwrap();
            }
            Bounce::Backward => {
                counter -= 1;
                pwm.set_duty_b(100 - counter).unwrap();
            }
        }
        pwm.set_config(&c);
    }
}
