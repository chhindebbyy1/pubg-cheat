use winapi::um::winuser::{GetWindowRect, RECT};

pub fn draw_esp() {
    let hwnd = FindWindowA(ptr::null_mut(), "PUBG");
    if hwnd.is_null() {
        return;
    }
    
    let mut rect: RECT = unsafe { std::mem::zeroed() };
    unsafe {
        GetWindowRect(hwnd, &mut rect);
    }
    
    let width = rect.right - rect.left;
    let height = rect.bottom - rect.top;
    
    draw_box(rect.left, rect.top, width, height);
}

fn draw_box(x: i32, y: i32, width: i32, height: i32) {
    // Drawing logic here
}