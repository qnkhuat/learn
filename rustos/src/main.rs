#![no_std] // There are no std in our own OS
#![no_main] // Disable RUST default entry points

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

// Entry point
#[no_mangle] // Making sure compile doesn't mangle the function name
// we choose _start as entrypoint bc it's the entry point name of most system
// pub extern "C" => tell compiler that it should use C calling convention
pub extern "C" fn _start() -> ! { 
  let vga_buffer = 0xb8000 as *mut u8; // Address of VGA buffer

  for (i, &byte) in HELLO.iter().enumerate() {
    unsafe {
      *vga_buffer.offset(i as isize * 2) = byte;
      *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
    }
  }
  loop {}
}

// This function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {  // ! is "never" type
  loop {}
}
