//! Windows yerel parola diyalogu (Explorer `--secure` parity, Faz 4 beta).

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum NativePromptOutcome {
    Accepted(String),
    Cancelled,
    Unavailable,
}

#[cfg(windows)]
mod win32 {
    use std::sync::Mutex;

    use windows::core::w;
    use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
    use windows::Win32::System::LibraryLoader::GetModuleHandleW;
    use windows::Win32::UI::WindowsAndMessaging::{
        CreateWindowExW, DefWindowProcW, DestroyWindow, DispatchMessageW, GetMessageW,
        GetWindowTextLengthW, GetWindowTextW, PostQuitMessage, RegisterClassW, ShowWindow,
        TranslateMessage, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, ES_PASSWORD, MSG, SW_SHOW,
        WINDOW_EX_STYLE, WINDOW_STYLE, WM_CLOSE, WM_COMMAND, WM_CREATE, WM_DESTROY, WNDCLASSW,
        WS_BORDER, WS_CAPTION, WS_CHILD, WS_POPUP, WS_SYSMENU, WS_TABSTOP, WS_VISIBLE,
    };

    const ID_OK: usize = 1002;
    const ID_CANCEL: usize = 1003;

    static PROMPT_TEXT: Mutex<Option<String>> = Mutex::new(None);
    static RESULT: Mutex<Option<Option<String>>> = Mutex::new(None);
    static EDIT_HWND: Mutex<Option<HWND>> = Mutex::new(None);

    unsafe extern "system" fn wnd_proc(
        hwnd: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        match msg {
            WM_CREATE => {
                let prompt = PROMPT_TEXT
                    .lock()
                    .ok()
                    .and_then(|g| g.clone())
                    .unwrap_or_else(|| "Arşiv parolası:".into());
                let prompt_u16: Vec<u16> = prompt.encode_utf16().chain([0]).collect();
                let _ = CreateWindowExW(
                    WINDOW_EX_STYLE::default(),
                    w!("Static"),
                    windows::core::PCWSTR(prompt_u16.as_ptr()),
                    WINDOW_STYLE(WS_CHILD | WS_VISIBLE),
                    12,
                    12,
                    360,
                    20,
                    hwnd,
                    None,
                    GetModuleHandleW(None).ok(),
                    None,
                );
                if let Ok(edit) = CreateWindowExW(
                    WINDOW_EX_STYLE(WS_BORDER.0),
                    w!("Edit"),
                    w!(""),
                    WINDOW_STYLE(WS_CHILD | WS_VISIBLE | WS_TABSTOP | ES_PASSWORD),
                    12,
                    40,
                    360,
                    24,
                    hwnd,
                    None,
                    GetModuleHandleW(None).ok(),
                    None,
                ) {
                    if let Ok(mut slot) = EDIT_HWND.lock() {
                        *slot = Some(edit);
                    }
                }
                let _ = CreateWindowExW(
                    WINDOW_EX_STYLE::default(),
                    w!("Button"),
                    w!("Tamam"),
                    WINDOW_STYLE(WS_CHILD | WS_VISIBLE | WS_TABSTOP),
                    200,
                    80,
                    80,
                    28,
                    hwnd,
                    windows::Win32::Foundation::HMENU(ID_OK as _),
                    GetModuleHandleW(None).ok(),
                    None,
                );
                let _ = CreateWindowExW(
                    WINDOW_EX_STYLE::default(),
                    w!("Button"),
                    w!("İptal"),
                    WINDOW_STYLE(WS_CHILD | WS_VISIBLE | WS_TABSTOP),
                    292,
                    80,
                    80,
                    28,
                    hwnd,
                    windows::Win32::Foundation::HMENU(ID_CANCEL as _),
                    GetModuleHandleW(None).ok(),
                    None,
                );
                LRESULT(0)
            }
            WM_COMMAND => {
                let id = wparam.0 as usize;
                if id == ID_OK {
                    if let Ok(guard) = EDIT_HWND.lock() {
                        if let Some(edit) = *guard {
                            let len = GetWindowTextLengthW(edit) as usize;
                            let mut buf = vec![0u16; len + 1];
                            let read = GetWindowTextW(edit, &mut buf);
                            if read > 0 {
                                let text = String::from_utf16_lossy(&buf[..read as usize]);
                                if let Ok(mut g) = RESULT.lock() {
                                    *g = Some(if text.is_empty() { None } else { Some(text) });
                                }
                            }
                        }
                    }
                    let _ = DestroyWindow(hwnd);
                } else if id == ID_CANCEL {
                    if let Ok(mut g) = RESULT.lock() {
                        *g = Some(None);
                    }
                    let _ = DestroyWindow(hwnd);
                }
                LRESULT(0)
            }
            WM_CLOSE => {
                if let Ok(mut g) = RESULT.lock() {
                    *g = Some(None);
                }
                let _ = DestroyWindow(hwnd);
                LRESULT(0)
            }
            WM_DESTROY => {
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcW(hwnd, msg, wparam, lparam),
        }
    }

    pub fn prompt(title: &str, message: &str) -> super::NativePromptOutcome {
        unsafe {
            if let Ok(mut g) = PROMPT_TEXT.lock() {
                *g = Some(message.to_string());
            }
            if let Ok(mut g) = RESULT.lock() {
                *g = None;
            }
            if let Ok(mut g) = EDIT_HWND.lock() {
                *g = None;
            }

            let class_name = w!("WinlrtPasswordDlg");
            let wc = WNDCLASSW {
                lpfnWndProc: Some(wnd_proc),
                hInstance: GetModuleHandleW(None).ok().into(),
                lpszClassName: class_name,
                style: CS_HREDRAW | CS_VREDRAW,
                ..Default::default()
            };
            let _ = RegisterClassW(&wc);

            let title_u16: Vec<u16> = title.encode_utf16().chain([0]).collect();
            let hwnd = CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                class_name,
                windows::core::PCWSTR(title_u16.as_ptr()),
                WINDOW_STYLE(WS_POPUP | WS_CAPTION | WS_SYSMENU),
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                400,
                150,
                None,
                None,
                GetModuleHandleW(None).ok(),
                None,
            );
            let Ok(hwnd) = hwnd else {
                return super::NativePromptOutcome::Unavailable;
            };
            let _ = ShowWindow(hwnd, SW_SHOW);

            let mut msg = MSG::default();
            while GetMessageW(&mut msg, None, 0, 0).into() {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }

            let outcome = RESULT
                .lock()
                .ok()
                .and_then(|g| g.clone())
                .flatten();
            match outcome {
                Some(p) => super::NativePromptOutcome::Accepted(p),
                None => super::NativePromptOutcome::Cancelled,
            }
        }
    }
}

#[cfg(windows)]
pub fn prompt_archive_password(title: &str, message: &str) -> NativePromptOutcome {
    if std::env::var("WINLRT_DISABLE_NATIVE_PASSWORD")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
    {
        return NativePromptOutcome::Unavailable;
    }
    win32::prompt(title, message)
}

#[cfg(not(windows))]
pub fn prompt_archive_password(_title: &str, _message: &str) -> NativePromptOutcome {
    NativePromptOutcome::Unavailable
}
