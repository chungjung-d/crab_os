#![no_std]
#![no_main]


use core::panic::PanicInfo;




fn main() {
}



/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}