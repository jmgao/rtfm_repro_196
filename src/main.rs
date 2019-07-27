#![no_main]
#![no_std]

extern crate panic_semihosting;

use stm32f1xx_hal::prelude::*;
use stm32f1xx_hal::gpio;

use rtfm::app;
use rtfm::Instant;

#[app(device = stm32f1xx_hal::stm32)]
const APP: () = {
  static mut LED: gpio::gpioc::PC13<gpio::Output<gpio::PushPull>> = ();

  #[init(schedule = [blink])]
  fn init() {
    let mut flash = device.FLASH.constrain();
    let mut rcc = device.RCC.constrain();

    let clocks = rcc
      .cfgr
      .use_hse(8.mhz())
      .sysclk(72.mhz())
      .pclk1(24.mhz())
      .freeze(&mut flash.acr);

    assert!(clocks.usbclk_valid());

    let mut gpioc = device.GPIOC.split(&mut rcc.apb2);
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    led.set_low();

    schedule.blink(Instant::now() + 72_000_000.cycles()).unwrap();

    LED = led;
  }

  #[task(schedule = [blink], resources = [LED])]
  fn blink() {
    resources.LED.toggle();
    schedule.blink(scheduled + 72_000_000.cycles()).unwrap();
  }

  extern "C" {
    fn EXTI0();
  }
};
