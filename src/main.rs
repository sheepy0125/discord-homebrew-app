/// Discord for the 3DS
extern crate ctru;
extern crate ctru_sys as libctru;

use ctru::applets::swkbd::{Button, Swkbd};
use ctru::console::Console;
use ctru::gfx::{Gfx, Screen};
use ctru::services::apt::Apt;
use ctru::services::hid::{Hid, KeyPad};
use ctru::services::soc::Soc;

use std::io::{Read, Write};
use std::net::TcpStream;

/// Constants
const PREFIX: &str = "!";
const BUFFER_SIZE: usize = 8 as usize;

/* Functions */
/// Shows a horizontal line as a border
fn border_line() -> String {
    let mut line = String::new();
    for _ in 0..80 {
        line.push('-');
    }
    line
}

/// Welcome message and instructions
fn welcome_message() {
    println!("Discord for the Nintendo 3DS!!!");
    println!("Press 'start' to exit.");
    println!("Press 'A' to type a message.");
    println!("Press 'select' to send the message.");
    println!("Press 'Y' to see this message again.");
}

/// Run
fn main() {
    let apt = Apt::init().unwrap();
    let hid = Hid::init().unwrap();
    let gfx = Gfx::default();
    let _console = Console::default();

    // The top screen will be used to show the messages
    let top_screen = Console::init(Screen::Top);
    // The bottom screen will be used to show the keyboard and status messages
    let bottom_screen = Console::init(Screen::Bottom);

    bottom_screen.select();
    welcome_message();
    border_line();

    // Connect to Discord
    println!("Connecting to Discord Proxy...");
    println!("Initializing network...");
    match Soc::init() {
        Ok(soc) => {
            println!("Successfully initialized Soc service");
            let proxy_ip = "192.168.86.67:7000";
            match TcpStream::connect(proxy_ip) {
                Ok(mut stream) => {
                    println!("Success! Sending hello...");
                    let hello_message = b"HELLO3DS";
                    stream.write(hello_message).unwrap();
                    println!("Waiting for response...");
                    let mut data = [0; 8]; // Length is 8
                    match stream.read_exact(&mut data) {
                        Ok(response) => {
                            if &data == hello_message {
                                println!("Hello succeeded!");
                            } else {
                                println!(
                                    "Hello failed! Got {} instead!",
                                    String::from_utf8_lossy(&data)
                                );
                            }
                        }
                        Err(e) => println!("Failed handshake: {}", e),
                    }
                }
                Err(e) => println!("Failed to connect: {}", e),
            }
        }
        Err(e) => println!("Failed to initialize Soc service: {}", e),
    }

    while apt.main_loop() {
        gfx.flush_buffers();
        gfx.swap_buffers();
        gfx.wait_for_vblank();

        hid.scan_input();
        let inputs = hid.keys_down();
        // Show a keyboard for the message input
        if inputs.contains(KeyPad::KEY_A) {
            let mut keyboard = Swkbd::default();
            let mut text = String::new();
            // TODO: fix scope
            match keyboard.get_utf8(&mut text) {
                Ok(Button::Right) => println!("Message: {}", text),
                Ok(Button::Left) => println!("Cancelled"),
                Ok(Button::Middle) => {
                    println!("what do you want me to do? save and not save the message?")
                }
                Err(_) => println!("Error!"),
            }
        }
        // Send the message
        if inputs.contains(KeyPad::KEY_SELECT) {
            // TODO: send the message
            println!("Message sent!");
        }
        // Exit
        if inputs.contains(KeyPad::KEY_START) {
            break;
        }
    }
}
