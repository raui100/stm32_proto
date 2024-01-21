# STM32F401 & [Embassy](https://embassy.dev/)
Playing around with an STM32F401.

## Blinking LED while pressing button
> cargo run --release --bin button_flash

- Running two tasks concurrently on single processor
- Using shared `AtomicBool` in two tasks
- Timing the duration of pressing the button
- Printing dynamic debug information

## Blinking LED
> cargo run --release --bin hello_world

Implemented a blinking LED (the `Hello World!` of the micro cosmos).