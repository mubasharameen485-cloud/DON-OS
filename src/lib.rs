// src/lib.rs

#![no_std]
#![feature(abi_x86_interrupt)] // Yeh line bohot zaroori hai!

use core::panic::PanicInfo;

pub mod services;

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    crate::serial_println!("[DON-OS KERNEL PANIC] - Oh no, Boss!");
    crate::serial_println!("{}", info);
    loop {}
}