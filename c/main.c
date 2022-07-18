#include "pico/stdlib.h"
#include "pico_delegates.h"
#include "ws2812b.pio.h"


/**
 * @brief The frequency of the WS2812B strip (800 kHz)
 */
#define WS2812B_FREQUENCY (800000)
/**
 * @brief The pins to use for the eight state machines 0:0, 0:1, 0:2, 0:3, 1:0, 1:1, 1:2, 1:3
 */
#define WS2812B_PINS {18, 19, 20, 21}


/**
 * @brief Loads the `ws2812b` program into PIO0 and initializes PIO0's state machines
 */
void pico_delegates_init_pio() {
    static bool is_init = false;
    if (!is_init) {
        // Get the state machine pins as array
        const uint pins[] = WS2812B_PINS;
        
        // Program pio 0
        uint program_offset_pio0 = pio_add_program(pio0, &ws2812b_program);
        ws2812b_program_init(pio0, 0, program_offset_pio0, pins[0], WS2812B_FREQUENCY);
        ws2812b_program_init(pio0, 1, program_offset_pio0, pins[1], WS2812B_FREQUENCY);
        ws2812b_program_init(pio0, 2, program_offset_pio0, pins[2], WS2812B_FREQUENCY);
        ws2812b_program_init(pio0, 3, program_offset_pio0, pins[3], WS2812B_FREQUENCY);
        
        // Set init flag
        is_init = true;
    }
}


/**
 * @brief The rust main function
 */
extern void rust_entry();


int main() {
    // Initialize context
    pico_delegates_init();
    pico_delegates_init_pio();
    
    // Enter rust
    rust_entry();
    return 0;
}
