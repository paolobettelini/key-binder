use keybinder::*;

use std::thread;

use std::process::Command;

fn main() {
    let mut handler = KeyBinder::new();

    // Press Home to open 'konsole'
    // Press End to open 'firefox'

    handler.always_ignore(vec![LOCK_MASK, MOD2_MASK, MOD3_MASK, MOD5_MASK]);

    handler.register(keysym::XK_End, Trigger::Press, vec![], open_firefox);

    handler.register(keysym::XK_Home, Trigger::Press, vec![], open_konsole);

    handler.listen();
}

fn open_firefox() {
    let _ = Command::new("firefox").output();
}

fn open_konsole() {
    thread::spawn(|| {
        let _ = Command::new("konsole").output();
    });
}