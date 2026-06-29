// src/main.rs

#![no_std]
#![no_main]
#![allow(unused_imports)]

use my_rust_os::services;
use my_rust_os::serial_println;
use bootloader::{entry_point, BootInfo};

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static BootInfo) -> ! {
    
    serial_println!("=====================================");
    serial_println!("   DON-OS Backend Server Starting... ");
    serial_println!("=====================================");
    
    // 1. Load the CPU Error Handler
    services::cpu_service::init();
    serial_println!("[INFO] CPU Exception Handler (IDT) Loaded!");

    // 2. VGA par text likhte hain (Frontend zinda hai)
    let vga_buffer = 0xb8000 as *mut u8;
    let message = b"DON-OS IS ALIVE!";
    for (i, &byte) in message.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0x0A; 
        }
    }

    // 3. THE REAL TEST: Hacking the Memory!
    serial_println!("[TEST] Triggering a Page Fault (Invalid Memory Access)...");
    
    unsafe {
        // 0xdeadbeef ek fake (invalid) memory address hai. 
        // Hum is par '42' likhne ki koshish kar rahe hain. CPU isko foran pakar lega!
        *(0xdeadbeef as *mut u64) = 42;
    };

    serial_println!("Yeh line kabhi print nahi hogi kyunke error catch ho jayega!");

    loop {}
}