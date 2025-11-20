#![no_std] //removes std which is the standard library (unavailable in baremetal)
#![no_main] //removes main which cant exist without std 

//library import block
use core::panic::PanicInfo;
use bootloader::entry_point;
use bootloader::BootInfo;

entry_point!(kernel_main); //the function that runs first in the code

//the first function
fn kernel_main(_boot_info: &'static BootInfo) -> ! {
  loop {} //keeps the cpu busy so no auto shutdown
}

//the panic handler
//triggers when there is an error
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  loop {}
}


