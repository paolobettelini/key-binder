pub use x11::keysym;
use x11::xlib;
use std::os::raw::{c_int, c_uint};
use std::ptr;

pub unsafe fn grab_without_mask(display: *mut xlib::Display, root: xlib::Window, key: c_uint) {
    grab_with_mask(display, root, key, 0 as c_uint);
}

pub unsafe fn grab_with_mask(display: *mut xlib::Display, root: xlib::Window, key: c_uint, mask: c_uint) {
    let key = key as xlib::KeySym;
    let keycode = xlib::XKeysymToKeycode(display, key) as c_int;
    
    xlib::XGrabKey(display,
                   keycode,
                   mask,
                   root,
                   true as c_int,
                   xlib::GrabModeAsync,
                   xlib::GrabModeAsync);
}

pub unsafe fn get_default_display() -> *mut xlib::Display {
    xlib::XOpenDisplay(ptr::null())
}

pub unsafe fn get_root(display: *mut xlib::Display) -> xlib::Window {
    xlib::XDefaultRootWindow(display)
}

pub unsafe fn next_event(display: *mut xlib::Display, event: &mut xlib::XEvent) {
    xlib::XNextEvent(display, event);
}

pub fn get_keysym(press: &mut xlib::XKeyEvent) -> xlib::KeySym {
    unsafe { xlib::XLookupKeysym(press, 0) }
}

pub fn get_state(press: &mut xlib::XKeyEvent) -> c_uint {
    press.state
}