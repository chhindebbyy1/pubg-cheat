use std::ptr;
use std::thread;
use std::time::Duration;
use winapi::um::winuser::{FindWindowA, GetWindowThreadProcessId, SetForegroundWindow, ShowWindow, SW_SHOW};
use rand::Rng;

fn main() {
    let hwnd = FindWindowA(ptr::null_mut(), "PUBG");
    if hwnd.is_null() {
        println!("PUBG is not running.");
        return;
    }
    
    let mut pid: u32 = 0;
    unsafe {
        GetWindowThreadProcessId(hwnd, &mut pid);
    }
    
    loop {
        let aimbot_active = true;
        if aimbot_active {
            aim_at_target(hwnd);
        }
        thread::sleep(Duration::from_millis(100));
    }
}

fn aim_at_target(hwnd: *mut std::ffi::c_void) {
    unsafe {
        SetForegroundWindow(hwnd);
        ShowWindow(hwnd, SW_SHOW);
    }
    let target_x = rand::thread_rng().gen_range(0..1920);
    let target_y = rand::thread_rng().gen_range(0..1080);
    move_mouse(target_x, target_y);
}

fn move_mouse(x: i32, y: i32) {
    let input = winapi::um::winuser::INPUT {
        r#type: winapi::um::winuser::INPUT_MOUSE,
        mi: winapi::um::winuser::MOUSEINPUT {
            dx: x,
            dy: y,
            mouseData: 0,
            dwFlags: winapi::um::winuser::MOUSEEVENTF_MOVE,
            time: 0,
            dwExtraInfo: 0,
        },
    };
    unsafe {
        winapi::um::winuser::SendInput(1, &input, std::mem::size_of::<winapi::um::winuser::INPUT>() as i32);
    }
}