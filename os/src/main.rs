// Disable standard library
#![no_std]

// Do not use all Rust-level entries such as the'main' function as the program entry
#![no_main]

// Embed assembly
#![feature(llvm_asm)]

// Embed the entire assembly file
#![feature(global_asm)]

// Get message and print when panic!
#![feature(panic_info_message)]

#[macro_use]
mod console;
mod panic;
mod sbi;

global_asm!(include_str!("entry.asm"));




// pub fn console_putchar(ch: u8) {
//     let _ret: usize;
//     let arg0: usize = ch as usize;
//     let arg1: usize = usize = 0;
//     let arg2: usize = 0;
//     let which: usize = 1;
//     unsafe{
//         llvm_asm!("ecall"
//             : "={x10}" (_ret)
//             : "{x10}" (arg0), "{x11}" (arg1), "{x12}" (arg2), "{x17}" (which)
//             : "memory"
//             : "violatile" 
//         );  
//     }
// }

// override the _start function in crt0
#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    // console_putchar(b'O');
    // console_putchar(b'K');
    // console_putchar(b'\n');

    // loop {}

    println!("Hello rCore-Tutorial!");
    panic!("end of rust_main")
}
