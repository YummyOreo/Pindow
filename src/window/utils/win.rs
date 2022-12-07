use notify_rust;
use regex::Regex;
use windows::{
    Win32::Foundation::HANDLE,
    Win32::Foundation::{CloseHandle, HWND},
    Win32::System::ProcessStatus::K32GetProcessImageFileNameA,
    Win32::System::Threading::{OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION},
    Win32::UI::WindowsAndMessaging,
};

// Impl async for these
pub fn get_process_file(handle: HANDLE) -> Option<String> {
    let mut wierd_path: [u8; 512] = [0; 512];
    let path_len: u32;
    unsafe {
        path_len = K32GetProcessImageFileNameA(handle, &mut wierd_path);
    };
    if path_len == 0 {
        return None;
    }

    let wierd_path = String::from_utf8_lossy(&wierd_path[..path_len as usize]);
    let re = Regex::new(r"\\Device\\HarddiskVolume[0-9]+\\").unwrap();
    let path = re.replace_all(&wierd_path, "").to_string();

    Some(path)
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

pub fn get_windows() -> Vec<isize> {
    // re do this to be better:
    // https://stackoverflow.com/questions/210504/enumerate-windows-like-alt-tab-does
    let mut windows: Vec<isize> = vec![];
    unsafe {
        let mut child = WindowsAndMessaging::GetTopWindow(None);
        loop {
            if child == HWND(0) {
                break;
            };
            if WindowsAndMessaging::IsWindowVisible(child).as_bool() {
                windows.insert(0, child.0);
            }
            child = WindowsAndMessaging::GetWindow(child, WindowsAndMessaging::GW_HWNDNEXT);
        }
    }
    return windows;
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
