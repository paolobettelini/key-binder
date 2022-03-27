pub use x11::keysym;
use x11::xlib;
use std::os::raw::{c_int, c_uint};
use std::collections::HashMap;

mod xlib_binding;
mod combinatory;

// http://euklid.mi.uni-koeln.de/c/mirror/www.cs.curtin.edu.au/units/cg252-502/notes/lect5h1.html

// https://tronche.com/gui/x/xlib/events/mask.html

// https://tronche.com/gui/x/xlib/events/keyboard-pointer/keyboard-pointer.html

//pub const NoMask: c_uint = xlib::NoEventMask;
pub type Mask = c_uint;

pub const SHIFT_MASK: Mask = xlib::ShiftMask;
pub const LOCK_MASK: Mask = xlib::LockMask;
pub const CTRL_MASK: Mask = xlib::ControlMask;
pub const MOD1_MASK: Mask = xlib::Mod1Mask;
pub const MOD2_MASK: Mask = xlib::Mod2Mask;
pub const MOD3_MASK: Mask = xlib::Mod3Mask;
pub const MOD4_MASK: Mask = xlib::Mod4Mask;
pub const MOD5_MASK: Mask = xlib::Mod5Mask;

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Hash)]
pub enum Trigger {
    Press,
    Release
}

impl Trigger {

    fn from_type(event_type: c_int) -> Self {
        return match event_type {
            xlib::KeyPress => Trigger::Press,
            xlib::KeyRelease => Trigger::Release,
            _ => Trigger::Release // ?
        }
    }

}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Hash)]
struct Binding<'a> {
    pub key: c_uint,
    pub trigger: &'a Trigger,
    pub mask: c_uint,
}

impl<'a> Binding<'a> {

    pub fn new(key: c_uint, trigger: &'a Trigger, mask: c_uint) -> Self {
        Binding {
            key: key,
            trigger: trigger,
            mask: mask
        }
    }

}

pub struct KeyBinder<'a> {
    display: *mut xlib::Display,
    root: xlib::Window,
    always_ignore: c_uint,
    press_bindings: HashMap<Binding<'a>, fn() -> ()>,
    release_bindings: HashMap<Binding<'a>, fn() -> ()>,
}

impl<'a> KeyBinder<'a> {

    pub fn new() -> Self {
        unsafe {
            let display = xlib_binding::get_default_display();
            let root = xlib_binding::get_root(display);
    
            KeyBinder {
                display: display,
                root: root,
                always_ignore: 0,
                press_bindings: HashMap::new(),
                release_bindings: HashMap::new()
            }
        }
    }

    pub fn always_ignore(&mut self, masks: Vec<c_uint>) {
        let mask = compact(&masks);

        self.always_ignore = mask;
    }


    pub fn register<'b: 'a>(&mut self, key: c_uint, trigger: &'b Trigger, masks: Vec<c_uint>, callback: fn() -> ()) {
        let mask = compact(&masks);

        // Register to all combinations of always_ignore_mask (+ the actual mask)
        let ignore_indexes = combinatory::get_ones_indexes(self.always_ignore);
        for i in 0..combinatory::total_combinations(ignore_indexes.len()) {
            let combination = combinatory::map_indexes_combination(&ignore_indexes, i as u32);
            
            unsafe {
                // Subscrive to event
                xlib_binding::grab_with_mask(self.display, self.root, key, mask | combination);
            }
        }

        let binding = Binding::new(key_upper(key), trigger, mask);

        match trigger {
            Trigger::Press => {
                self.press_bindings.insert(binding, callback);
            }
            Trigger::Release => {
                self.release_bindings.insert(binding, callback);
            }
        }
    }

    pub fn listen(&mut self) {
        let mut event = xlib::XEvent { pad: [0; 24] };
        loop {
            unsafe {
                xlib_binding::next_event(self.display, &mut event);
            }

            let event_type = event.get_type();
            let trigger = Trigger::from_type(event_type);
            let key = key_upper(xlib_binding::get_keysym((&mut event).as_mut()) as c_uint);
            let mut mask = xlib_binding::get_state((&mut event).as_mut());

            // Remove ignored masks
            mask &= !self.always_ignore;

            let binding = Binding::new(key, &trigger, mask);

            let callback = match event_type {
                xlib::KeyPress => self.press_bindings.get(&binding),
                xlib::KeyRelease => self.release_bindings.get(&binding),
                _ => None
            };

            if let Some(f) = callback {
                f();
            }
        }
    }

}

/// Convert a vector of masks into a single mask
fn compact(masks: &Vec<c_uint>) -> c_uint {
    let mut result = 0;

    for mask in masks {
        result |= *mask;
    }

    result
}

/// Make letter uppercase
fn key_upper(key: u32) -> u32 {
    if key >= 'a' as u32 && key <= 'z' as u32 {
        return key - 'a' as u32 + 'A' as u32;
    }
    key
}
