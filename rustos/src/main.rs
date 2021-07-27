#![no_std] // There are no std in our own OS
#![no_main] // Disable RUST default entry points

use core::panic::PanicInfo;

mod vga_buffer;

static HELLO: &[u8] = b"Hello World!";

// Entry point
#[no_mangle] // Making sure compile doesn't mangle the function name
// we choose _start as entrypoint bc it's the entry point name of most system
// pub extern "C" => tell compiler that it should use C calling convention
pub extern "C" fn _start() -> ! { 
  println!("Hello World{}", "!");
  loop {}
}

// This function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {  // ! is "never" type
  println!("{}", info);
  loop {}
}
