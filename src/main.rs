use winsafe::{prelude::*, Sleep};
use winsafe::{co::MB, co::WS, co::WS_EX};
use winsafe::HWND;
use rdev::listen;

fn main() {
    println!("Hello, world!");

    let mut last_focused: HWND = winsafe::HWND::NULL;
    let mut subject: HWND = winsafe::HWND::NULL;

    fn callback(event: rdev::Event) {
        match event.event_type {
            rdev::EventType::KeyPress(key) => match key {
                rdev::Key::KeyH => {
                    println!("key: {:?}", key);
                },
                rdev::Key::KeyJ => {
                    println!("key: {:?}", key);
                },
                _ => {}
            },
            _ => {}
        }
    }
    match listen(callback) {
        Ok(_) => println!("listen ok"),
        Err(_) => println!("listen error"),
    }
    // Set subject on keypress
    //
    // Check if hotkey is pressed
    // If yes:
    // - Get focused window
    // - Check if focused window is subject
    // - If yes:
    // -- Set focused window to last_focused window if it exists
    // - If no:
    // -- Set last_focused to focused window
    // -- Set focused window to subject
    // If no:
    // - Do nothing
    //
    // Sleep for some time
    // Repeat
    loop {
        let woah = match winsafe::HWND::GetForegroundWindow() {
            Some(focused) => {
                println!("focused: {:?}", focused);
                focused
            }
            None => {
                println!("Error: No focused window found.");
                winsafe::HWND::NULL
            }
        };
        if woah == winsafe::HWND::NULL {
            println!("woah is null");
        }
        println!("woah: {:?}", woah);
        println!("is_focused(&woah): {:?}", is_focused(&woah));
        // assert!(is_focused(&woah));

        Sleep(1);
    }
}

fn is_focused(hwnd: &HWND) -> bool {
    match winsafe::HWND::GetForegroundWindow() {
        Some(focused) => {
            return &focused == hwnd;
        }
        None => {
            println!("Error: No focused window found.");
            return false;
        }
    }
}
