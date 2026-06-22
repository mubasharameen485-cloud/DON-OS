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
    serial_println!("[INFO] Bootloader successfully passed control to Kernel.");
    serial_println!("[INFO] IO Service Loaded Successfully!");
    serial_println!("[SUCCESS] Boss, your Kernel is ALIVE! ");

   
    loop {}
}