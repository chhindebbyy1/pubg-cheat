use winapi::um::winuser::{CreateWindowExA, WS_OVERLAPPEDWINDOW, ShowWindow, SW_SHOW};
use std::ptr;

pub fn create_window() {
    let class_name = "PUBG Cheat\0".as_ptr() as *const i8;
    let hwnd = unsafe {
        CreateWindowExA(
            0,
            class_name,
            class_name,
            WS_OVERLAPPEDWINDOW,
            100,
            100,
            300,
            200,
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
        )
    };
    unsafe {
        ShowWindow(hwnd, SW_SHOW);
    }
}