//! Implements the WS2812B strip handler

use crate::config::{WS2812B_LED_MAX, WS2812B_STATEMACHINES};
use alloc::vec::Vec;
use core::cmp::max;
use picolib::{
    error::Error,
    pio::StateMachine,
    sync::{Lock, Mutex},
    thread,
};

/// A 3*8-bit RGB value
#[derive(Debug, Clone, Copy)]
pub struct Rgb {
    /// The red value
    pub r: u8,
    /// The green value
    pub g: u8,
    /// The blue value
    pub b: u8,
}
impl Rgb {
    /// Creates a new RGB value
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
    /// Creates a new all-zero RGB value
    pub const fn zero() -> Self {
        Self { r: 0, g: 0, b: 0 }
    }

    /// `self` as 0-green-red-blue 32 bit integer
    pub fn to_0grb(self) -> u32 {
        (self.g as u32) << 16 | (self.r as u32) << 8 | (self.b as u32)
    }
}

/// A WS2812B matrix
#[derive(Debug, Clone, Copy)]
pub struct Matrix {
    /// The amount of LEDs to address
    pub len: usize,
    /// The RGB values
    pub rgb: [Rgb; WS2812B_LED_MAX],
}
impl Matrix {
    /// Creates a new all-zero WS2812B matrix
    pub const fn zero() -> Self {
        Self { len: 0, rgb: [Rgb::zero(); WS2812B_LED_MAX] }
    }

    /// Sets an RGB value for an LED index
    pub fn set(&mut self, index: usize, rgb: Rgb) -> Result<(), Error> {
        // Validate the index and select the LED
        let led = self.rgb.get_mut(index).ok_or(error!("LED index is out of bounds"))?;

        // Update the matrix
        self.len = max(self.len, index + 1);
        *led = rgb;
        Ok(())
    }
}

/// The lock handle
static _INBOX_LOCK: Lock = Lock::new();
/// The threadsafe inbox handle for core 1
static INBOX: Mutex<'static, Vec<Matrix>> = Mutex::new(&_INBOX_LOCK, Vec::new());

/// The entry point for core 1
extern "C" fn core1_entry() {
    // Start state machines
    let mut state_machines = Vec::with_capacity(WS2812B_STATEMACHINES);
    for machine in 0..WS2812B_STATEMACHINES {
        // Create state machine handle
        let state_machine = StateMachine::new(0, machine as u32).expect("Failed to create state machine?!");
        state_machines.push(state_machine);
    }

    // Enter runloop
    let mut state = vec![Matrix::zero(); WS2812B_STATEMACHINES];
    loop {
        // Copy the inbox into the core-local cache
        INBOX.synchronized(|_state| state.copy_from_slice(_state));

        // Feed all state machines
        for led in 0..WS2812B_LED_MAX {
            // Feed all state machines
            'sm_loop: for machine in 0..WS2812B_STATEMACHINES {
                // Check if we have reached the end of the strip
                let len = state[machine].len;
                if led >= len {
                    continue 'sm_loop;
                }

                // Write the RGB value
                let rgb = state[machine].rgb[led];
                state_machines[machine].write(rgb.to_0grb());

                // Write the reset command if appropriate
                if led + 1 == len {
                    state_machines[machine].write(u32::MAX);
                }
            }
        }
    }
}

/// Starts the strip handler on core 1 and returns the inbox handle
pub fn start() -> &'static Mutex<'static, Vec<Matrix>> {
    // Halt core 1 and prepare the inbox
    thread::core1_halt();
    INBOX.synchronized(|state| *state = vec![Matrix::zero(); WS2812B_STATEMACHINES]);

    // Start core 1 and return the inbox handle
    thread::core1_start(core1_entry);
    &INBOX
}
