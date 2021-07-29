#![no_std] // There are no std in our own OS
#![no_main] // Disable RUST default entry points
#![feature(custom_test_frameworks)]
#![test_runner(rustos::test_runner)]
#![reexport_test_harness_main = "test_main"] // redefine test's entry point so we can conditionally call test in test_context

use rustos::println;
use core::panic::PanicInfo;

// we choose _start as entrypoint bc it's the entry point name of most system
// pub extern "C" => tell compiler that it should use C calling convention
#[no_mangle] // Making sure compile doesn't mangle the function name
pub extern "C" fn _start() -> ! { 
  println!("Hello World{}", "!");

  rustos::init();

  // trigger a page fault
  unsafe {
    *(0xdeadbeef as *mut u64) = 42;
  }

  fn stack_overflow() {
    stack_overflow(); // for each recursion, the return address is pushed
  }

  // trigger a stack overflow
  //stack_overflow();

  #[cfg(test)]
  test_main();
  println!("It did not crash!");

  loop {}
}

// This function is called on panic
#[cfg(not(test))] // run when not in test context
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {  // ! is "never" type
  println!("{}", info);
  loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  rustos::test_panic_handler(info)
}
