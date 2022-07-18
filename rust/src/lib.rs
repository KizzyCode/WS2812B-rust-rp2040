#![no_std]
#![doc = include_str!("../../README.md")]
#![feature(alloc_error_handler)]

#[macro_use]
extern crate picolib;
#[macro_use]
extern crate alloc;

mod command;
mod config;
mod ws2812b;

use crate::{
    config::{COMMAND_MAX, WS2812B_STATEMACHINES},
    ws2812b::Matrix,
};
use alloc::vec::Vec;
use core::{alloc::Layout, panic::PanicInfo};
use picolib::{error, memory::PicoMalloc};

/// Handles a panic
#[panic_handler]
fn panic_handler(panic: &PanicInfo) -> ! {
    error::panic_handler(panic)
}

/// Handles an allocation error
#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    panic!("Allocation error for: {layout:?}")
}

/// The global allocator
#[global_allocator]
static ALLOCATOR: PicoMalloc = PicoMalloc;

/// The main entry function
#[no_mangle]
pub extern "C" fn rust_entry() {
    // Start the second core
    let inbox = ws2812b::start();

    // Read the commands
    let mut linebuf = Vec::with_capacity(COMMAND_MAX);
    let mut state = vec![Matrix::zero(); WS2812B_STATEMACHINES];

    'runloop: loop {
        // Read a command
        if let Err(e) = command::parse(&mut linebuf, &mut state) {
            let _ = println!("ERROR ({e})");
            continue 'runloop;
        }

        // Send the updated state to the inbox
        inbox.synchronized(|_state| _state.copy_from_slice(&state));
        let _ = println!("OK");
    }
}
