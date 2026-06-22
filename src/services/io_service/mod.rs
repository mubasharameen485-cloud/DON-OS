// src/services/io_service/mod.rs

pub mod serial;


#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::services::io_service::serial::_print(format_args!($($arg)*));
    };
}


#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
        concat!($fmt, "\n"), $($arg)*
    ));
}