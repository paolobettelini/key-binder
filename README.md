# Key Binder

A simple Xorg keybinder written in Rust, using the X11 crate.

Key pressed are detected globally.

# Try the example

```bash
    git clone https://github.com/paolobettelini/key-binder
    cd key-binder
    cargo run --example simple
```

# Usage

```rust
use keybinder::*;

fn main() {
    let mut handler = KeyBinder::new();

    handler.always_ignore(vec![LOCK_MASK, MOD2_MASK, MOD3_MASK, MOD5_MASK]);

    handler.register(keysym::XK_F, &Trigger::Release, vec![SHIFT_MASK, CTRL_MASK], shift_ctrl_f);

    handler.listen();
}

fn shift_ctrl_f() {
    println!("shift+ctrl+f pressed!");
}
```