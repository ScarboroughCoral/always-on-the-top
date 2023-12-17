use std::{fmt::Error};
use windows::{
    core,
    Win32::{UI::WindowsAndMessaging::*, Foundation::*}
};
fn is_window_on_desktop(hwnd: HWND) -> bool {
    let mut placement: WINDOWPLACEMENT = WINDOWPLACEMENT::default() ;
    if (unsafe { GetWindowPlacement(hwnd, &mut placement).is_ok() }) {
        return placement.showCmd == SW_SHOWNORMAL.0 as u32
    }
    return false;
}

#[derive(serde::Serialize)]
pub struct WindowDetail {
    title: String,
    hwnd: isize,
}
static mut WINDOWS: Vec<WindowDetail> = Vec::new();
pub fn get_all_windows() -> Result<Vec<WindowDetail>, Error> {

    Ok(unsafe { &WINDOWS }.iter().map(|detail| WindowDetail { title: detail.title.clone(), hwnd: detail.hwnd }).collect())
}

extern "system" fn enum_windows_proc(hwnd: HWND, _l_param: LPARAM) -> BOOL {
    if (unsafe { IsWindowVisible(hwnd).as_bool() }) {
        let mut title: Vec<u16> = vec![0; 255];
        let title_length = unsafe { GetWindowTextW(hwnd, &mut title) };
        if (title_length > 0) {
            unsafe {
                WINDOWS.push(WindowDetail { title: String::from_utf16_lossy(&title[0..(title_length as usize)]), hwnd: hwnd.0 });
            }
        }
    }
    return BOOL(1);
}
pub fn init()-> Result<(), core::Error> {
    unsafe { EnumWindows(Option::Some(enum_windows_proc), LPARAM(0)) }
}