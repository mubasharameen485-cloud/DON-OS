// src/main.rs

#![no_std]
#![no_main]

// Hum apni library (lib.rs) se components import kar rahe hain
use my_rust_os::services;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // ---------------------------------------------------------
    // Yahan DON-OS ka boot sequence start hoga.
    // Future mein hum yahan likhenge:
    // services::io_service::init();
    // services::memory_service::init();
    // ---------------------------------------------------------
    
    // OS ko zinda rakhne ke liye infinite loop
    loop {}
}