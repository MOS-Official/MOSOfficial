#![no_std] //removes std which is the standard library (unavailable in baremetal)
#![no_main] //removes main which cant exist without std 
#![feature(alloc_error_handler)] //this i think fixes create alloc not found am not sure though but it makes it compile so 
//some creates like alloc for teh allocator etc
extern crate alloc;

//modularization of files
mod allocator;

//library import block
use core::panic::PanicInfo;
use bootloader::entry_point;
use bootloader::BootInfo;

use alloc::vec::Vec;

//this is my own crate so the allocator gets imported USE IT
use crate::allocator::ALLOCATOR;

entry_point!(kernel_main); //the function that runs first in the code

//the first function
#[unsafe(no_mangle)] //just so the name doesnt change with magic or smt i dont know just read i have to put this
fn kernel_main(_boot_info: &'static BootInfo) -> ! {
  //NOW WE CAN FINALLY USE HASHMAPS <3
  //AM HAPPY
  unsafe { //rust is a bitch
    ALLOCATOR.set_heap(0x_4444_0000, 1024 * 1024); //1MB heap i dont understand the math behind it but if we want more good luck
  }
  
  loop {} //keeps the cpu busy so no auto shutdown
}

//the panic handler
//triggers when there is an error
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  loop {}
}


