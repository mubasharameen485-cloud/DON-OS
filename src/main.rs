// src/main.rs

#![no_std]   // Rule 1: Standard library use nahi karni
#![no_main]  // Rule 2: Normal 'main' function use nahi karna (Is line ko nahi mitana)

use core::panic::PanicInfo;

/// Yeh function tab call hoga jab hamara OS crash karega.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Yeh hamare Kernel ka asal Entry Point (Darwaza) hai.
/// Naye Rust Nightly ke rules ke mutabiq hum #[unsafe(no_mangle)] use karenge.
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // OS yahan zinda rahega aur infinite loop mein ghoomta rahega
    loop {}
}