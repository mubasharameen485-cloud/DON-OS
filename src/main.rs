
#![no_std]
#![no_main]


use my_rust_os::services;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    
    loop {}
}