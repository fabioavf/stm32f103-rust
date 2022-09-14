#![no_std]
#![no_main]

use cortex_m_rt::entry; // The runtime
use embedded_hal::digital::v2::{InputPin, OutputPin}; // the `set_high/low`function

#[allow(unused_imports)]
use panic_halt;
use stm32f1xx_hal::{delay::Delay, pac, prelude::*}; // STM32F1 specific functions // When a panic occurs, stop the microcontroller

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain();
    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);

    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

    let (pa15, pb3, pb4) = afio.mapr.disable_jtag(gpioa.pa15, gpiob.pb3, gpiob.pb4);

    let mut led0 = gpioa.pa0.into_push_pull_output(&mut gpioa.crl); //w
    let mut led1 = gpioa.pa1.into_push_pull_output(&mut gpioa.crl); //w
    let mut led2 = gpioa.pa2.into_push_pull_output(&mut gpioa.crl); //w
    let mut led3 = pa15.into_push_pull_output(&mut gpioa.crh);
    let mut led4 = gpioa.pa8.into_push_pull_output(&mut gpioa.crh);
    let mut led5 = gpioa.pa6.into_push_pull_output(&mut gpioa.crl); //w
    let mut led6 = gpioa.pa5.into_push_pull_output(&mut gpioa.crl); //w
    let mut led7 = gpioa.pa11.into_push_pull_output(&mut gpioa.crh);

    let mut sw0 = gpiob.pb12.into_pull_down_input(&mut gpiob.crh);

    let mut buzz = gpiob.pb0.into_push_pull_output(&mut gpiob.crl);

    let mut flash = dp.FLASH.constrain();

    let clocks = rcc.cfgr.sysclk(8.mhz()).freeze(&mut flash.acr);

    let mut delay = Delay::new(cp.SYST, clocks);

    loop {
        // led0.set_high().ok();
        // delay.delay_ms(50_u16);

        // led1.set_high().ok();
        // delay.delay_ms(50_u16);

        // led2.set_high().ok();
        // delay.delay_ms(50_u16);

        // led3.set_high().ok();
        // delay.delay_ms(50_u16);

        // led4.set_high().ok();
        // delay.delay_ms(50_u16);

        // led5.set_high().ok();
        // delay.delay_ms(50_u16);

        // led6.set_high().ok();
        // delay.delay_ms(50_u16);

        // led7.set_high().ok();
        // delay.delay_ms(50_u16);

        // led0.set_low().ok();
        // delay.delay_ms(50_u16);

        // led1.set_low().ok();
        // delay.delay_ms(50_u16);

        // led2.set_low().ok();
        // delay.delay_ms(50_u16);

        // led3.set_low().ok();
        // delay.delay_ms(50_u16);

        // led4.set_low().ok();
        // delay.delay_ms(50_u16);

        // led5.set_low().ok();
        // delay.delay_ms(50_u16);

        // led6.set_low().ok();
        // delay.delay_ms(50_u16);

        // led7.set_low().ok();
        // delay.delay_ms(50_u16);

        if (sw0.is_low().ok().unwrap()) {
            led0.set_high().ok();
        } else {
            led0.set_low().ok();
        }
    }
}
