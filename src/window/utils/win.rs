use notify_rust;
use windows::{
    Win32::Foundation::{HANDLE, HINSTANCE, CloseHandle, HWND},
    Win32::System::ProcessStatus::K32GetModuleFileNameExA,
    Win32::System::Threading::{OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION},
    Win32::UI::WindowsAndMessaging,
};

// Impl async for these
pub fn get_process_file(handle: HANDLE) -> Option<String> {
    let mut wierd_path: [u8; 512] = [0; 512];
    let path_len: u32;
    unsafe {
        path_len = K32GetModuleFileNameExA(handle, HINSTANCE(0), &mut wierd_path);
    };
    if path_len == 0 {
        return None;
    }

    let path = String::from_utf8_lossy(&wierd_path[..path_len as usize]);

    Some(path.to_string())
}

pub fn get_id(window: isize) -> u32 {
    let mut id: u32 = 0;
    unsafe {
        WindowsAndMessaging::GetWindowThreadProcessId(
            HWND { 0: window },
            Option::Some(&mut id as *mut u32),
        );
    };

    id
}
pub fn get_handle(id: u32) -> Option<HANDLE> {
    //! REMEMBER TO CLOSE THE HANDLE WITH `close_handle(Handle);`
    unsafe {
        let handler_wraped = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, id);
        match handler_wraped {
            Ok(handle) => Some(handle),
            _ => None,
        }
    }
}
pub fn close_handle(handle: HANDLE) {
    unsafe {
        CloseHandle(handle);
    };
}

pub fn current_window() -> isize {
    unsafe { WindowsAndMessaging::GetForegroundWindow().0 }
}

pub fn popup(title: String, message: String) {
    // replace this with better solution
    let _ = notify_rust::Notification::new()
        .appname("Pin App")
        .icon("file:///C:/Users/OreoD/Downloads/pin.png")
        .summary(&title)
        .body(&message)
        .timeout(notify_rust::Timeout::Milliseconds(1000))
        .show();
}
