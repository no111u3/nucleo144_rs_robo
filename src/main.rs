#![no_std]
#![no_main]

use panic_semihosting as _;

use rtfm::app;

use stm32f7x7_hal as _;

#[app(device = stm32f7x7_hal::stm32)]
const APP: () = {
    #[init]
    fn init() {}
};
