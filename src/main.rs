//! Testing PWM output

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_std]
#![feature(lang_items, start)]

extern crate cortex_m;
extern crate panic_abort;
extern crate stm32f103xx_hal as hal;

use cortex_m::asm;
use hal::prelude::*;
use hal::stm32f103xx;

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    let p = stm32f103xx::Peripherals::take().unwrap();

    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio = p.AFIO.constrain(&mut rcc.apb2);

    // let mut gpioa = p.GPIOA.split(&mut rcc.apb2);
    let mut gpiob = p.GPIOB.split(&mut rcc.apb2);

    // TIM2
    // let c1 = gpioa.pa0.into_alternate_push_pull(&mut gpioa.crl);
    // let c2 = gpioa.pa1.into_alternate_push_pull(&mut gpioa.crl);
    // let c3 = gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl);
    // let c4 = gpioa.pa3.into_alternate_push_pull(&mut gpioa.crl);

    // TIM3
    // let c1 = gpioa.pa6.into_alternate_push_pull(&mut gpioa.crl);
    // let c2 = gpioa.pa7.into_alternate_push_pull(&mut gpioa.crl);
    // let c3 = gpiob.pb0.into_alternate_push_pull(&mut gpiob.crl);
    // let c4 = gpiob.pb1.into_alternate_push_pull(&mut gpiob.crl);

    // TIM4
    let c1 = gpiob.pb6.into_alternate_push_pull(&mut gpiob.crl);
    let c2 = gpiob.pb7.into_alternate_push_pull(&mut gpiob.crl);
    let c3 = gpiob.pb8.into_alternate_push_pull(&mut gpiob.crh);
    let c4 = gpiob.pb9.into_alternate_push_pull(&mut gpiob.crh);

    let mut pwm = p.TIM4
        .pwm(
            (c1, c2, c3, c4),
            &mut afio.mapr,
            1.khz(),
            clocks,
            &mut rcc.apb1,
        )
        .3;

    let max = pwm.get_max_duty();

    pwm.enable();

    // full
    pwm.set_duty(max);

    asm::bkpt();

    // dim
    pwm.set_duty(max / 4);

    asm::bkpt();

    // zero
    pwm.set_duty(0);

    asm::bkpt();
    0
}
