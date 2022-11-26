use regex::Regex;
use windows::{
    Win32::Foundation::HANDLE,
    Win32::Foundation::{CloseHandle, BOOL, HWND},
    Win32::System::ProcessStatus::K32GetProcessImageFileNameA,
    Win32::System::Threading::{OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION},
    Win32::UI::WindowsAndMessaging,
};
use winapi::{shared::minwindef::LPARAM, shared::windef, um::winuser::EnumWindows};

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

pub struct ActivateWindowError {}
impl std::fmt::Display for ActivateWindowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Could not activate window!")
    }
}

pub fn activate_window(window: isize) -> Result<(), ActivateWindowError> {
    let success: BOOL;
    unsafe {
        success = WindowsAndMessaging::SetForegroundWindow(HWND { 0: window });
    };
    match success {
        BOOL(0) => Err(ActivateWindowError {}),
        _ => Ok(()),
    }
}

pub fn get_windows() -> Vec<isize> {

    extern "system" fn enum_windows_proc(hwnd: windef::HWND, l_param: LPARAM) -> i32 {
        /* unsafe { */
        unsafe {
            let windows: &mut Vec<isize> = &mut *(l_param as *mut Vec<isize>);
            /* windows.as_ref().unwrap().clone().insert(0, *hwnd.cast()); */
            windows.insert(0, hwnd as isize);
            /* windows.to_vec() */
            /* println!("{:?}", hwnd as isize); */

            *(l_param as *mut Vec<isize>) = windows.to_vec().clone();
        };

            /* windows.insert(0, *(hwnd).cast()); */
            /* *(l_param as *mut Vec<isize>) = windows.to_vec(); */
        /* }; */
        true.into()
    }

    let mut windows: Vec<isize> = vec![];
    unsafe {
        EnumWindows(
            Some(enum_windows_proc),
            &mut windows as *mut Vec<isize> as LPARAM,
        );
    };
    return windows;
}
