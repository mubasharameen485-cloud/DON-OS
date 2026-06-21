// src/lib.rs

#![no_std] // Rule 1: No Standard Library

use core::panic::PanicInfo;

// Hamara main services folder yahan register hoga
pub mod services;

/// Yeh hamara Global Panic Handler hai. Jab OS crash hoga toh yeh chalega.
/// (Jab hum IO Service mukammal kar lenge, toh hum yahan error message print karwayenge)
#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}