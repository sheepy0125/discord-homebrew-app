/// Discord for the 3DS
/* Imports */
extern crate ctru;
extern crate ctru_sys as libctru;

use ctru::applets::swkbd::{Button, Swkbd};
use ctru::console::Console;
use ctru::gfx::{Gfx, Screen};
use ctru::services::{
    apt::Apt,
    hid::{Hid, KeyPad},
    soc::Soc,
};

use std::io::{Read, Write};
use std::net::TcpStream;

/* Constants */
const BUFFER_SIZE: usize = 1024 as usize;
const TOP_SCREEN_COLS: u8 = 50;
const BOTTOM_SCREEN_COLS: u8 = 40;

/* Functions */
/// Shows a horizontal line as a border
fn border_line(length: u8) {
    let mut line = String::new();
    for _ in 0..length {
        line.push('-');
    }
    println!("{}", line);
}

/// Welcome message and instructions
fn welcome_message() {
    println!("Discord for the Nintendo 3DS!!!");
    println!("Press 'Y' to see this message again.");
    println!("Press 'A' to type a message.");
    println!("Press 'select' to send the message.");
    println!("Press 'start' to exit.");
}

/// Run
fn main() {
    let apt = Apt::init().unwrap();
    let hid = Hid::init().unwrap();
    let gfx = Gfx::default();
    let _console = Console::default();
    let mut message = String::new();
    let mut proxy_ip = String::new();

    // The top screen will be used to show the messages
    let top_screen = Console::init(Screen::Top);
    // The bottom screen will be used to show the keyboard and status messages
    let bottom_screen = Console::init(Screen::Bottom);

    bottom_screen.select();
    welcome_message();
    border_line(BOTTOM_SCREEN_COLS);

    // Connect to Discord proxy
    println!("Connecting to Discord Proxy...");
    println!("Initializing network...");

    // Get the IP address of the proxy
    top_screen.select();
    println!("Please enter the IP address of the Discord Proxy:");
    let mut keyboard = Swkbd::default();
    let mut text = String::new();
    match keyboard.get_utf8(&mut text) {
        Ok(Button::Right) => {
            println!("IP address: {}", text);
            proxy_ip.push_str(&text);
        }
        Ok(Button::Left) => println!("Cancelled"),
        Ok(Button::Middle) => println!("Cancelled"),
        Err(e) => println!("Error getting text: {:?}", e),
    }
    top_screen.clear();
    bottom_screen.select();

    match Soc::init() {
        Ok(_) => {
            println!("Successfully initialized Soc service");
            match TcpStream::connect(proxy_ip.as_str()) {
                Ok(mut stream) => {
                    println!("Success! Sending hello...");
                    const HELLO_MESSAGE: &[u8; 8] = b"HELLO3DS";
                    stream.write(HELLO_MESSAGE).unwrap();
                    println!("Waiting for response...");
                    let mut data = [0; HELLO_MESSAGE.len()];
                    match stream.read_exact(&mut data) {
                        Ok(_) => {
                            if &data == HELLO_MESSAGE {
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

                    top_screen.select();
                    border_line(TOP_SCREEN_COLS);
                    println!("Welcome to the Discord for the Nintendo 3DS!");
                    border_line(TOP_SCREEN_COLS);
                    bottom_screen.select();

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
                            match keyboard.get_utf8(&mut text) {
                                Ok(Button::Right) => {
                                    println!("Message: {}", text);
                                    message.clear();
                                    message.push_str(&text);
                                }
                                Ok(Button::Left) => println!("Cancelled"),
                                Ok(Button::Middle) => println!("Cancelled"),
                                Err(e) => println!("Error getting text: {:?}", e),
                            }
                        }
                        // Send the message
                        if inputs.contains(KeyPad::KEY_SELECT) {
                            println!("Sending message...");
                            let message = format!("SEND{}", message);
                            stream.write(message.as_bytes()).unwrap();
                            println!("Message sent! Awaiting response...");
                            let mut data = [0 as u8; BUFFER_SIZE];
                            match stream.read(&mut data) {
                                Ok(size) => {
                                    if &data[0..size] == b"SENT" {
                                        println!("Success!");
                                    } else {
                                        println!(
                                            "Message failed! Got {} instead!",
                                            String::from_utf8_lossy(&data)
                                        );
                                    }
                                }
                                Err(e) => println!("Failed to send message: {}", e),
                            }
                        }
                        // Exit
                        if inputs.contains(KeyPad::KEY_START) {
                            panic!("Exiting Discord for the 3DS! Press OK to exit.");
                        }

                        // Detect if there is a new message
                        let mut data = [0 as u8; 2048];
                        match stream.write(b"GET") {
                            Ok(_) => match stream.read(&mut data) {
                                Ok(size) => {
                                    let data_str =
                                        String::from_utf8_lossy(&data[0..size]).to_string();
                                    if data_str.starts_with("MESSAGE") {
                                        let message = data_str[("MESSAGE".len())..].to_string();
                                        top_screen.select();
                                        println!("{}", message);
                                        border_line(TOP_SCREEN_COLS);
                                        bottom_screen.select();
                                    }
                                }
                                Err(e) => println!("Failed to read message: {}", e),
                            },
                            Err(e) => println!("Failed to send GET for messages: {}", e),
                        }
                    }
                }
                Err(e) => println!("Failed to connect: {}", e),
            }
        }
        Err(e) => println!("Failed to initialize Soc service: {}", e),
    }
}
