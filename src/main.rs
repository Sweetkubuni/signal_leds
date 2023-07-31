use rppal::gpio::{Gpio};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::io;

const LED_PIN: u8 = 2; // Replace with the actual GPIO pin number you're using for the LED

fn main() {
    // Get the GPIO instance
    let gpio = Gpio::new().unwrap();

    // Setup LED pin as an output
    let mut led_pin = gpio.get(LED_PIN).unwrap().into_output();

    // Create a channel to communicate with the LED thread
    let (sender, receiver) = mpsc::channel();

    // Spawn a new thread to handle the LED toggling
    let led_thread = thread::spawn(move || {
        loop {
            // Check if there is a message to stop the LED thread
            if let Ok(stop) = receiver.try_recv() {
                if stop {
                    break;
                }
            }

            // Toggle the LED on
            led_pin.set_high();
            thread::sleep(Duration::from_millis(200));

            // Toggle the LED off
            led_pin.set_low();
            thread::sleep(Duration::from_millis(200));
        }
    });

    // Your main application code
    // Simulate an error for demonstration purposes
    simulate_error();

    // Prompt for key input before stopping the LED thread
    println!("Press Enter to stop the LED thread.");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // Send a message to stop the LED thread
    sender.send(true).unwrap();

    // Wait for the LED thread to finish
    let _ = led_thread.join();
}

// Simulate an error in your application code
fn simulate_error() {
    println!("Simulating an error!");
    // Your error-handling logic here
}

