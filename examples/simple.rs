use keybinder::*;

fn main() {
    let mut handler = KeyBinder::new();

    handler.always_ignore(vec![LOCK_MASK, MOD2_MASK, MOD3_MASK, MOD5_MASK]);

    handler.register(keysym::XK_F, &Trigger::Release, vec![SHIFT_MASK, CTRL_MASK], shift_ctrl_f);

    handler.register(keysym::XK_F, &Trigger::Release, vec![SHIFT_MASK], shift_f);

    handler.register(keysym::XK_F, &Trigger::Release, vec![CTRL_MASK], ctrl_f);

    handler.listen();
}

fn shift_f() {
    println!("shift+f");
}

fn ctrl_f() {
    println!("ctrl+f");
}

fn shift_ctrl_f() {
    println!("shift+ctrl+f");
}