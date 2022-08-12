//! Some config constants

/// The maximum LEDs within a LED strip
pub const WS2812B_LED_MAX: usize = 70;
/// The amount of PIO statemachines that can address a WS2812B LED strip
pub const WS2812B_STATEMACHINES: usize = 4;
/// The maximum size of a single transaction
pub const TRANSACTION_MAX: usize = 4096;
