use std::{
    ffi::CString,
    mem::zeroed,
    os::raw::{c_char, c_int, c_long, c_uchar, c_uint, c_ulong},
    ptr,
};
use window_wrapper::x11;
fn main() {
    println!("Hello, world!");

    unsafe {
        let display = x11::XOpenDisplay(ptr::null());

        let screen = x11::XDefaultScreen(display);
        let root = x11::XDefaultRootWindow(display);

        let mut attrs: x11::XSetWindowAttributes = zeroed();
        attrs.background_pixel = x11::XWhitePixel(display, screen);

        let window = x11::XCreateWindow(
            display,
            root,
            0,
            0,
            600,
            400,
            0,
            x11::CopyFromParent as c_int,
            x11::InputOutput as c_uint,
            ptr::null_mut(),
            x11::CWBackPixel as c_ulong,
            &mut attrs,
        );

        // set window title
        let title_str = CString::new("hello-world").unwrap();
        x11::XStoreName(display, window, title_str.as_ptr() as *mut c_char);

        // Hook close request
        let wm_protocols_str = CString::new("WM_PROTOCOLS").unwrap();
        let wm_delete_window_str = CString::new("WM_DELETE_WINDOW").unwrap();

        let wm_protocols =
            x11::XInternAtom(display, wm_protocols_str.as_ptr(), x11::False as c_int);
        let wm_delete_window =
            x11::XInternAtom(display, wm_delete_window_str.as_ptr(), x11::False as c_int);

        let mut protocols = [wm_delete_window];

        x11::XSetWMProtocols(
            display,
            window,
            protocols.as_mut_ptr(),
            protocols.len() as c_int,
        );

        // KeyInput
        x11::XSelectInput(
            display,
            window,
            (x11::ExposureMask | x11::KeyPressMask) as c_long,
        );

        // Show window
        x11::XMapWindow(display, window);

        let mut event: x11::XEvent = zeroed();

        loop {
            x11::XNextEvent(display, &mut event);
            match event.type_ as u32 {
                x11::ClientMessage => {
                    let xclient = event.xclient;

                    if xclient.message_type == wm_protocols && xclient.format == 32 {
                        let protocol = xclient.data.l[0] as x11::Atom;

                        if protocol == wm_delete_window {
                            break;
                        }
                    }
                }
                x11::KeyPress => {
                    let keysym = x11::XKeycodeToKeysym(display, event.xkey.keycode as c_uchar, 0);

                    if keysym == x11::XK_Escape as c_ulong {
                        break;
                    }
                }
                _ => (),
            }
        }
    }
}
