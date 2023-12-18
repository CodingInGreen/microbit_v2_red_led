#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print};
use panic_rtt_target as _;
use microbit::hal::gpio::{p0::Parts as P0Parts, Level};
use microbit::hal::pac::Peripherals;
use embedded_hal::digital::v2::OutputPin;

#[entry]
fn main() -> ! {
    // RTT is a feature provided by the `rtt-target` crate, which is a Rust implementation of a real-time data transfer
    // protocol designed for use with embedded systems. This protocol enables efficient, low-overhead logging and
    // data transfer between a microcontroller (like the micro:bit) and a host computer.

    // The macro `rtt_init_print!()` sets up the necessary infrastructure for the microcontroller to send output
    // (like debug messages or print statements) over RTT. This is particularly useful in a `no_std` environment, 
    // where traditional output methods like standard output or file logging are not available.

    // After calling `rtt_init_print!()`, you can use functions like `rprintln!` or `rprint!` to send messages
    // from the microcontroller to the connected host computer. These messages can be viewed using tools that
    // support RTT, such as the SEGGER J-Link software or other compatible debugging tools.

    // This line is essential for debugging and monitoring the behavior of your program, especially during development
    // and testing. By providing a way to output messages, it allows for better insight into the program's execution,
    // which is invaluable in an embedded context where direct observation of program behavior can be challenging.
    rtt_init_print!();

    // This line attempts to take ownership of all the peripheral devices on the micro:bit.
    // `Peripherals::take()` returns an instance of the `Peripherals` struct which contains
    // all the peripheral instances (like GPIO, timers, etc.) that the micro:bit supports.
    // `expect` is used to handle the case where `take()` returns `None`, which would happen if
    // something else had already taken ownership of the peripherals.
    // In this case, "Couldn't initialize peripherals." will be printed and the program will halt.
    let peripherals = Peripherals::take().expect("Couldn't initialize peripherals.");

    // This line initializes the GPIO pins of the micro:bit. 
    // `P0Parts::new` is a method that takes the GPIO port 0 (`P0`) from the peripherals
    // and returns an instance (`gpio`) that provides access to individual pins of the port.
    // `gpio` can then be used to configure each GPIO pin (e.g., as input, output, pull-up, pull-down).
    let gpio = P0Parts::new(peripherals.P0);


    // This line configures a specific GPIO pin (in this case, pin P0_03) for use as a digital output.
    // 'gpio.p0_03' accesses the GPIO pin P0_03 from the 'gpio' instance (which represents all GPIO pins of port P0).
    // The method 'into_push_pull_output(Level::Low)' is then called on this pin. This method does two things:
    // 1. It configures the pin as a push-pull output. In push-pull mode, the pin can actively drive the output high
    //    (to the voltage level of the microcontroller, usually around 3.3V for a micro:bit) or low (to ground, 0V).
    //    This mode is often used for driving LEDs, as it provides a more robust signal compared to open-drain or open-source configurations.
    // 2. It sets the initial output level of the pin to 'Level::Low', which means the pin will start in a state that corresponds to 0 volts.
    //    As a result, if an LED were connected to this pin (with the other LED lead connected to a higher voltage like 3.3V),
    //    it would start off being off (not lit), as the voltage difference across the LED would be zero or near-zero.
    //
    // 'let mut red_led' declares a mutable variable 'red_led' and assigns this configured GPIO pin to it.
    // The mutability ('mut') allows the state of this pin to be changed later in the code (for example, turning the LED on or off).
    //
    // Pinout reference: https://tech.microbit.org/hardware/edgeconnector/#pins-and-signals
    // p0_p3 will actually be physical pin 1 on the Kitronik Edge Connector Breakout Board.
    let mut red_led = gpio.p0_03.into_push_pull_output(Level::Low);
    
    // This line of code is responsible for turning on the LED connected to the GPIO pin represented by `red_led`.
    // The method `set_high()` is called on the `red_led` variable. This action attempts to set the output level of the pin to high,
    // which, for most microcontrollers, translates to a voltage near the supply voltage (e.g., 3.3V for the micro:bit).
    // Setting the pin high would create a voltage difference across the LED (assuming the other LED lead is connected to a lower voltage or ground),
    // causing current to flow through the LED and making it emit light.
    //
    // The `set_high()` method is part of the `OutputPin` trait from the `embedded_hal` crate, a hardware abstraction layer
    // used in embedded Rust to provide a consistent interface for interacting with GPIO pins across different microcontrollers.
    //
    // The `.expect("Could not set the LED high")` part is a way of handling potential errors. The `set_high()` method can fail
    // (for example, if the pin is not properly configured as an output, or if there's a hardware fault). If an error occurs,
    // `expect` will cause the program to panic and halt, displaying the message "Could not set the LED high".
    // This is a simple way to catch and report errors in scenarios where recovery from such an error is not straightforward or not critical
    // for the basic functioning of the program.
    //
    // In summary, this line is crucial for the LED's operation in the circuit. 
    // If the line executes successfully, the LED will turn on.
    red_led.set_high().expect("Could not set the LED high");

    loop {
        // Infinite Loop 
        //The LED stays on as long as the micro:bit V2 is powered
    }
}