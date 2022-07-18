/**
 * @package picolib
 * @file pico_delegates.h
 * @brief This file defines the delegate API contract
 */

#ifndef PICO_DELEGATES_H
#define PICO_DELEGATES_H

#include <stdint.h>


/**
 * @brief Initializes the pico delegates implementation
 * 
 * @warning This function must be called before any other delegate function is used
 */
void pico_delegates_init();


/**
 * @brief A buffer to write a panic message into
 */
extern volatile uint8_t pico_panic_buf[256];
/**
 * @brief Calls the SDK panic handler
 * 
 * @param message The panic message
 */
void __attribute__((noreturn)) pico_panic();


/**
 * @brief Allocates some memory
 * 
 * @param ptr The target pointer
 * @param size The amount of bytes to allocate
 */
void pico_mem_alloc(uint8_t** ptr, uint32_t size);
/**
 * @brief Reallocates some memory
 * 
 * @param ptr The target pointer to reallocate
 * @param size The new size to reallocate
 */
void pico_mem_realloc(uint8_t** ptr, uint32_t size);
/**
 * @brief Frees some allocated memory
 * 
 * @param ptr The target pointer to free
 */
void pico_mem_free(uint8_t** ptr);


/**
 * @brief Gets a char from stdin
 * 
 * @param result The target pointer
 */
void pico_stdio_getc(uint8_t* result);
/**
 * @brief Gets a char from stdin
 * 
 * @param result The target pointer
 * @param us The timeout in milliseconds
 * @return `0` on success or `-1` in case of a timeout
 */
int32_t pico_stdio_getc_timeout(uint8_t* result, uint32_t us);
/**
 * @brief Writes a char to stdout
 * 
 * @param value The char to write
 */
void pico_stdio_putc(uint8_t value);


/**
 * @brief Sleeps the given amount of milliseconds
 * 
 * @param ms The amount of milliseconds to sleep
 */
void pico_sleep_ms(uint32_t ms);
/**
 * @brief Sleeps the given amount of microseconds
 * 
 * @param us The amount of milliseconds to sleep
 */
void pico_sleep_us(uint32_t us);


/**
 * @brief Launches some code function on core 1
 * 
 * @param entry The entry point
 */
void pico_core1_start(void(*entry)());
/**
 * @brief Stops core 1
 */
void pico_core1_halt();


/**
 * @brief Locks the shared spinlock
 * 
 * @param irq A pointer store the current interrupt states
 */
void pico_spinlock_lock(uint32_t* irq);
/**
 * @brief Unlocks the shared spinlock
 * 
 * @param irq The interrupt states to restore
 */
void pico_spinlock_unlock(uint32_t irq);


/**
 * @brief Starts a state machine
 * 
 * @param pio The PIO index
 * @param sm The state machine index
 * 
 * @warning Please your C code is responsible to initialize the PIOs their state machines as appropriate
 */
void pico_piosm_start(uint32_t pio, uint32_t sm);
/**
 * @brief Stops a state machine 
 * 
 * @param pio The PIO index
 * @param sm The state machine index
 * 
 * @note It is intentional that this library does not provide a start function because PIO initialization is extremely
 *       implementation dependant. You need to implement and invoke your initializer yourself.
 */
void pico_piosm_halt(uint32_t pio, uint32_t sm);
/**
 * @brief Gets a value from a state machine queue
 * 
 * @param result The target pointer
 * @param pio The PIO index
 * @param sm The state machine index
 * @return `0` on success or `-1` in case of an error
 */
void pico_piosm_get(uint32_t* result, uint32_t pio, uint32_t sm);
/**
 * @brief Puts a value to a state machine queue
 * 
 * @param pio The PIO index
 * @param sm The state machine index
 * @param value The value
 */
void pico_piosm_put(uint32_t pio, uint32_t sm, uint32_t value);


/**
 * @brief Initializes a GPIO pin
 * 
 * @param pin The pin to initialize
 * @param direction The pin direction (i.e. `0` to read, `1` to write)
 */
void pico_gpio_init(uint32_t pin, uint8_t direction);
/**
 * @brief Gets the state of a GPIO pin
 * 
 * @param value The target pointer (i.e. `0` for low, `1` to high)
 * @param pin The pin to read
 */
void pico_gpio_get(uint8_t* value, uint32_t pin);
/**
 * @brief Sets the state of a GPIO pin
 * 
 * @param pin The pin to write
 * @param value The value to set (i.e. `0` for low, `1` to high)
 */
void pico_gpio_put(uint32_t pin, uint8_t value);


#endif // PICO_DELEGATES_H
