[![License BSD-2-Clause](https://img.shields.io/badge/License-BSD--2--Clause-blue.svg)](https://opensource.org/licenses/BSD-2-Clause)
[![License MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

# WS2812B
A driver implementation for up to 4 WS2812B LED strips. The heavy I/O-lifting is done via the PIO co-processors to
ensure timing correctness.

## Sending commands
The Pico accepts control commands via USB or UART serial at 115200 baud/s.
A command constists of
 - the LED strip index (0..4), followed by a colon
 - the LED index (0..WS2812B_LED_MAX), followed by a colon
 - the RGB-red value (0..256), followed by a colon
 - the RGB-green value (0..256), followed by a colon
 - the RGB-blue value (0..256), followed by a colon

Multiple commands can be chained into a space-separated list. A transaction is completed with a newline (`\n`).

Format example: `<strip>:<led>:<red>:<green>:<blue> <strip>:<led>:<red>:<green>:<blue> <...>\n`

Specific example: `0:0:255:0:0 0:1:0:255:0 0:2:0:0:255\n` - this sets the first LED to max red, the second LED to max
green and the third LED to max blue.

## Build hints
The C-Pico-SDK must be linked into `$PROJECT_DIR/sdk`. Furthermore, this project requires a Rust-nightly toolchain for
`thumbv6m-none-eabi`.
