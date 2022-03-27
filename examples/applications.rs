use keybinder::*;

use std::thread;

use std::process::Command;

fn main() {
    let mut handler = KeyBinder::new();

    // Press Home to open 'konsole'
    // Press End to open 'firefox'
    // Press Mod4+Enter to open 'rofi'

    handler.always_ignore(vec![LOCK_MASK, MOD2_MASK, MOD3_MASK]);

    handler.register(keysym::XK_End, Trigger::Press, vec![], start_firefox);
    handler.register(keysym::XK_Home, Trigger::Press, vec![], start_konsole);
    handler.register(keysym::XK_Return, Trigger::Press, vec![MOD4_MASK], start_rofi);

    handler.listen();
}

fn start_firefox() {
    let _ = Command::new("firefox").output();
}

fn start_konsole() {
    thread::spawn(|| {
        let _ = Command::new("konsole").output();
    });
}

fn start_rofi() {
    thread::spawn(|| {
        let _ = Command::new("rofi")
            .arg("-show")
            .arg("run")
            .output();
    });
}