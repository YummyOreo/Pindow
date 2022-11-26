// use std::process::Command;
//
pub mod handler;
mod update;
pub mod utils;

// extern "system" fn enum_window(window: HWND, _l: LPARAM) -> BOOL {
//     // make HWND by HWND{
//     //  0: NUMBER
//     // }
//     unsafe {
//         // println!("{l:?}");
//         let mut text: [u16; 512] = [0; 512];
//         let len = WindowsAndMessaging::GetWindowTextW(window, &mut text);
//         let _text = String::from_utf16_lossy(&text[..len as usize]);

//         let mut id: u32 = 0;
//         let raw_id = &mut id as *mut u32;
//         WindowsAndMessaging::GetWindowThreadProcessId(window, Option::Some(raw_id));

//         let handler = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, id);
//         // when implementing, if this does not work, then move on to the next one and mby remove it
//         // from the list
//         if handler.is_ok() {
//             let unwraped_handler = handler.unwrap();
//             // println!("{:?}", unwraped_handler);

//             let mut path: [u8; 512] = [0; 512];
//             let path_len = K32GetProcessImageFileNameA(unwraped_handler, &mut path);
//             let path = String::from_utf8_lossy(&path[..path_len as usize]);
//             let re = Regex::new(r"\\Device\\HarddiskVolume[0-9]+\\").unwrap();
//             let path_replaced = re.replace_all(&path, "");

//             // println!("{path_replaced:?}");
//             if path_replaced == "Program Files\\Alacritty\\alacritty.exe" {
//                 // try doing this first, if it fails (see documentation to know) then try
//                 // SetActiveWindow, if that fails, then it can't be activated and log to the
//                 // console *if it is in debug
//                 WindowsAndMessaging::SetForegroundWindow(window);
//             }

//             CloseHandle(unwraped_handler);
//         }

//         true.into()
//     }
// }
