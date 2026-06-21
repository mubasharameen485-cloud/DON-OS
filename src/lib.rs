
#![no_std] 

use core::panic::PanicInfo;

pub mod services;

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}