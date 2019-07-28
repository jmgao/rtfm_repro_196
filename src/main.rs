#![no_main]
#![no_std]

extern crate panic_semihosting;

use rtfm::app;
use rtfm::Instant;

pub use stm32f3xx_hal::prelude::*;
pub use stm32f3xx_hal::gpio;
use embedded_hal::digital::v2::OutputPin;

static mut ON: bool = true;

#[app(device = stm32f3xx_hal::stm32)]
const APP: () = {
  static mut LED: gpio::gpioc::PC13<gpio::Output<gpio::PushPull>> = ();

  #[init(schedule = [blink])]
  fn init() {
    let mut flash = device.FLASH.constrain();
    let mut rcc = device.RCC.constrain();

    let clocks = rcc
      .cfgr
      .sysclk(72.mhz())
      .pclk1(24.mhz())
      .freeze(&mut flash.acr);

    let mut gpioc = device.GPIOC.split(&mut rcc.ahb);
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.moder, &mut gpioc.otyper);
    led.set_low().unwrap();

    schedule.blink(Instant::now() + 72_000_000.cycles()).unwrap();

    LED = led;
  }

  #[task(schedule = [blink], resources = [LED])]
  fn blink() {
    unsafe {
      if ON {
        resources.LED.set_high().unwrap();
        ON = false;
      } else {
        resources.LED.set_low().unwrap();
        ON = true;
      }
    }
    schedule.blink(scheduled + 72_000_000.cycles()).unwrap();
  }

  extern "C" {
    fn EXTI0();
  }
};
