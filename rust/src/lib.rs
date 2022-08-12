#![no_std]
#![doc = include_str!("../../README.md")]
#![feature(alloc_error_handler)]

#[macro_use]
extern crate picolib;
#[macro_use]
extern crate alloc;

mod config;
mod transaction;
mod ws2812b;

use crate::{
    config::{TRANSACTION_MAX, WS2812B_STATEMACHINES},
    ws2812b::Matrix,
};
use alloc::vec::Vec;
use core::{alloc::Layout, panic::PanicInfo};
use picolib::{
    error,
    gpio::{Direction, Gpio},
    memory::PicoMalloc,
};

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
    // Init LED
    let mut led = Gpio::new(25, Direction::Write);
    let mut led_state = false;

    // Start the second core
    let inbox = ws2812b::start();

    // Read the transactions
    let mut linebuf = Vec::with_capacity(TRANSACTION_MAX);
    let mut state = vec![Matrix::zero(); WS2812B_STATEMACHINES];
    loop {
        // Read a transaction
        match transaction::parse(&mut linebuf, &mut state) {
            Ok(transaction_id) => {
                // Send the updated state to the inbox
                inbox.synchronized(|_state| _state.copy_from_slice(&state));
                let _ = println!("OK {transaction_id}");
            }
            Err(e) => {
                // Log the error and continue the runloop
                let _ = println!("ERROR {e}");
            }
        };

        // Toggle the LED
        led_state = !led_state;
        led.set(led_state);
    }
}
