// src/services/cpu_service/mod.rs

use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};
use lazy_static::lazy_static;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.double_fault.set_handler_fn(double_fault_handler);
        
        // NAYA: Page Fault (Memory Error) Handler
        idt.page_fault.set_handler_fn(page_fault_handler); 
        
        idt
    };
}

pub fn init() {
    IDT.load();
}

// 100% CRASH-PROOF PRINTER (Assembly)
fn emergency_print(msg: &str) {
    for byte in msg.bytes() {
        unsafe {
            core::arch::asm!(
                "out dx, al",
                in("dx") 0x3F8u16,
                in("al") byte,
            );
        }
    }
}

extern "x86-interrupt" fn breakpoint_handler(_stack_frame: InterruptStackFrame) {
    emergency_print("\n[EXCEPTION] BREAKPOINT CAUGHT!\n");
}

extern "x86-interrupt" fn double_fault_handler(_stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
    emergency_print("\n[FATAL] DOUBLE FAULT CAUGHT!\n");
    loop {} 
}

// NAYA: Page Fault Handler
extern "x86-interrupt" fn page_fault_handler(
    _stack_frame: InterruptStackFrame,
    _error_code: PageFaultErrorCode,
) {
    emergency_print("\n=====================================\n");
    emergency_print(" [SUCCESS] PAGE FAULT CAUGHT, BOSS!\n");
    emergency_print(" SOMEONE TRIED TO HACK THE MEMORY!\n");
    emergency_print("=====================================\n");
    
    // Memory error ke baad hum OS ko yahin freeze kar denge taa ke PC restart na ho
    loop {}
}