#[link(name = "EGL")]
#[link(name = "GLESv2")]
extern "C" {}

use std::{
    ffi::{CStr, CString},
    mem::zeroed,
    os::raw::{c_char, c_int, c_long, c_uchar, c_uint, c_ulong},
    ptr,
};
use window_wrapper::gles;
use window_wrapper::gles::{GLenum, GLint};
use window_wrapper::x11;

use egl;
use egl::{EGLNativeWindowType, EGLint};

const WIDTH: u32 = 960;
const HEIGHT: u32 = 480;

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
            WIDTH,
            HEIGHT,
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

        // setup EGL
        // 有効なDisplayExtentionを確認する
        let dp_extensions = {
            let p = egl::query_string(egl::EGL_NO_DISPLAY, egl::EGL_EXTENSIONS as i32).unwrap();
            let list = String::from_utf8(p.to_bytes().to_vec()).unwrap_or_else(|_| format!(""));
            list.split(' ').map(|e| e.to_string()).collect::<Vec<_>>()
        };
        let has_dp_extension = |e: &str| dp_extensions.iter().find(|s| s == &e).is_some();

        // x11のクライアントが利用可能であることを確認
        // x11の他にwaylandやsurfacelessがある
        if !(has_dp_extension("EGL_EXT_platform_x11") || has_dp_extension("EGL_KHR_platform_x11")){
            panic!("has not support x11")
        }

        let egl_display = egl::get_display(egl::EGL_DEFAULT_DISPLAY).unwrap();
        let mut major: EGLint = 0;
        let mut minor: EGLint = 0;
        egl::initialize(egl_display, &mut major, &mut minor);
        println!("EGL version is {}.{}", major, minor);

        let config_attrs: [EGLint; 1] = [egl::EGL_NONE];
        let config = egl::choose_config(egl_display, &config_attrs, 1).unwrap();

        let context_attrs: [EGLint; 3] = [egl::EGL_CONTEXT_CLIENT_VERSION, 2, egl::EGL_NONE];
        let egl_context =
            egl::create_context(egl_display, config, egl::EGL_NO_CONTEXT, &context_attrs).unwrap();

        let surface =
            egl::create_window_surface(egl_display, config, window as EGLNativeWindowType, &[])
                .unwrap();

        if !egl::make_current(egl_display, surface, surface, egl_context) {
            panic!("eglMakeCurrent == EGL_FALSE")
        }

        // link process
        // gl::load_with(|s| egl::get_proc_address(s) as _);

        // check gl command
        display_gl_capability();

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

// OpenGL関係の情報の表示 for debug
fn display_gl_capability() {
    let gpu = gl_get_string(gles::GL_RENDERER as gles::GLenum);
    let vendor = gl_get_string(gles::GL_VENDOR);
    let version = gl_get_string(gles::GL_VERSION);
    let shader_language_version = gl_get_string(gles::GL_SHADING_LANGUAGE_VERSION);
    // let extensions = gl_get_string(gl::EXTENSIONS);
    // let mut max_samples: GLint = 0;
    // unsafe {
    //     gl::GetIntegerv(gl::MAX_SAMPLES, &mut max_samples);
    // }

    println!("gpu: {:?}", gpu);
    println!("vendor: {:?}", vendor);
    println!("version: {:?}", version);
    println!("shader_language_version: {:?}", shader_language_version);
    // println!("extensions: {:?}", extensions);
    // println!("max multi samples: {:?}", max_samples);
}

#[cfg(target_arch = "x86_64")]
fn gl_get_string(name: GLenum) -> CString {
    unsafe {
        // GLに指示を出してCのstrのポインターを受け取る
        let version = gles::glGetString(name);
        // ポインタからCStrに変換して所有権のあるCStringにして返す
        CStr::from_ptr(version as *const i8).to_owned()
    }
}

#[cfg(target_arch = "aarch64")]
fn gl_get_string(name: GLenum) -> CString {
    unsafe {
        // GLに指示を出してCのstrのポインターを受け取る
        let version = gles::glGetString(name);
        // ポインタからCStrに変換して所有権のあるCStringにして返す
        CStr::from_ptr(version as *const u8).to_owned()
    }
}
