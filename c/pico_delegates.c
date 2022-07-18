/**
 * @package picolib
 * @file pico_delegates.c
 * @brief This file provides a default delegate implementation
 */

#include <stdio.h>
#include <malloc.h>
#include "pico/stdlib.h"
#include "pico/multicore.h"
#include "hardware/pio.h"
#include "pico_delegates.h"


/**
 * @brief A shared spinlock for locking purposes
 */
spin_lock_t* _pico_spinlock = NULL;
/**
 * @brief A shared boolean to check if the delegates have been initialized
 */
bool _pico_delegates_init = false;


void pico_delegates_init() {
    if (!_pico_delegates_init) {
        // Initialize stdio
        stdio_init_all();

        // Initialize the spinlock
        int pico_spinlock_index = spin_lock_claim_unused(true);
        _pico_spinlock = spin_lock_init(pico_spinlock_index);

        // Set init flag to true
        _pico_delegates_init = true;
    }
}


volatile uint8_t pico_panic_buf[256] = { 0 };
void __attribute__((noreturn)) pico_panic() {
    pico_panic_buf[255] = '\0';
    panic("%s", pico_panic_buf);
}


void pico_mem_alloc(uint8_t** ptr, uint32_t size) {
    *ptr = malloc(size);
    if (*ptr == NULL) {
        panic("Failed to allocate %lu bytes of memory", size);
    }
}
void pico_mem_realloc(uint8_t** ptr, uint32_t size) {
    *ptr = realloc(*ptr, size);
    if (*ptr == NULL) {
        panic("Failed to reallocate %lu bytes of memory", size);
    }
}
void pico_mem_free(uint8_t** ptr) {
    if (*ptr != NULL) {
        free(*ptr);
    }
    *ptr = NULL;
}


void pico_stdio_getc(uint8_t* result) {
    while (true) {
        // Get and validate the char
        const int c = getc(stdin);
        if (c >= 0 && c <= 255) {
            *result = (uint8_t)c;
            return;
        }
    }
}
int32_t pico_stdio_getc_timeout(uint8_t* result, uint32_t us) {
    // Get and validate the char
    const int c = getchar_timeout_us(us);
    if (c >= 0 && c <= 255) {
        *result = (uint8_t)c;
        return 0;
    }
    return -1;
}
void pico_stdio_putc(uint8_t value) {
    putc(value, stdout);
}


void pico_sleep_ms(uint32_t ms) {
    sleep_ms(ms);
}
void pico_sleep_us(uint32_t us) {
    sleep_us(us);
}


void pico_core1_start(void(*entry)()) {
    multicore_launch_core1(entry);
}
void pico_core1_halt() {
    multicore_reset_core1();
}


void pico_spinlock_lock(uint32_t* irq) {
    *irq = spin_lock_blocking(_pico_spinlock);
}
void pico_spinlock_unlock(uint32_t irq) {
    spin_unlock(_pico_spinlock, irq);
}


/**
 * @brief Resolves a numerical PIO index into an appropriate handle
 * 
 * @param target The target pointer
 * @param index The PIO index
 * @return int32_t `0` on success or `-1` in case of an error
 */
PIO _pico_pio_from_index(uint32_t index) {
    switch (index) {
        case 0: return pio0;
        case 1: return pio1;
        default: panic("Invalid PIO index %lu", index);
    }
}
void pico_piosm_start(uint32_t pio, uint32_t sm) {
    PIO _pio = _pico_pio_from_index(pio);
    pio_sm_claim(_pio, sm);
    pio_sm_set_enabled(_pio, sm, true);
}
void pico_piosm_halt(uint32_t pio, uint32_t sm) {
    PIO _pio = _pico_pio_from_index(pio);
    pio_sm_set_enabled(_pio, sm, false);
    pio_sm_unclaim(_pio, sm);
}
void pico_piosm_get(uint32_t* result, uint32_t pio, uint32_t sm) {
    PIO _pio = _pico_pio_from_index(pio);
    *result = pio_sm_get_blocking(_pio, sm);
}
void pico_piosm_put(uint32_t pio, uint32_t sm, uint32_t value) {
    PIO _pio = _pico_pio_from_index(pio);
    pio_sm_put_blocking(_pio, sm, value);
}


void pico_gpio_init(uint32_t pin, uint8_t direction) {
    gpio_init(pin);
    gpio_set_dir(pin, direction == 1);
}
void pico_gpio_get(uint8_t* value, uint32_t pin) {
    *value = gpio_get(pin) ? 1 : 0;
}
void pico_gpio_put(uint32_t pin, uint8_t value) {
    gpio_put(pin, value == 1);
}
