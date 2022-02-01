#![allow(non_snake_case)]

use core::ffi::c_void;
pub use windows::{
    core::Error,
    core::HRESULT,
    Win32::Foundation::*,
    Win32::Graphics::Gdi::PAINTSTRUCT,
    Win32::Graphics::Gdi::HFONT,
    Win32::Graphics::Gdi::HBRUSH,
    Win32::Graphics::Gdi::ValidateRect,
    Win32::Graphics::Gdi::EndPaint,
    Win32::Graphics::Gdi::BeginPaint,
    Win32::System::LibraryLoader::GetModuleHandleA,
    Win32::UI::WindowsAndMessaging::*,
    Win32::UI::Controls::*,
    Win32::System::Diagnostics::Debug::FormatMessageA,
    Win32::System::Diagnostics::Debug::FORMAT_MESSAGE_IGNORE_INSERTS,
    Win32::System::Diagnostics::Debug::FORMAT_MESSAGE_FROM_SYSTEM,
    Win32::System::Diagnostics::Debug::FORMAT_MESSAGE_ALLOCATE_BUFFER,
};
use crate::AnchorMap;
use crate::Notifier;
use crate::NotifierExt;

pub type MSG = windows::Win32::UI::WindowsAndMessaging::MSG;
#[allow(dead_code)]
pub static ES_AUTOHSCROLL: i32 = windows::Win32::UI::WindowsAndMessaging::ES_AUTOHSCROLL;
pub static WS_VSCROLL: WINDOW_STYLE = windows::Win32::UI::WindowsAndMessaging::WS_VSCROLL;

pub type ATOM = u16;

#[allow(dead_code)]
#[inline]
pub fn LOWORD(l: usize) -> usize {
    l & 0xffff
}
#[inline]
pub fn HIWORD(l: usize) -> usize {
    (l >> 16) & 0xffff
}

pub fn GetModuleHandle() -> HINSTANCE { unsafe { GetModuleHandleA(None) } }
pub fn LoadCursor() -> HCURSOR { unsafe { LoadCursorW(None, IDC_ARROW) } }

pub fn GetClassInfoEx(className: &str) -> Option<WNDCLASSEXA> {
    let hInstance = GetModuleHandle();

    let mut wcex = WNDCLASSEXA { ..Default::default() };

    unsafe {
        let res = GetClassInfoExA(hInstance, PSTR(className.as_ptr() as _), &mut wcex);

        if res == true {
            Some(wcex)
        } else {
            let error = HRESULT(GetLastError() as i32);

            println!("3) Error: {:?}. Message: {:?}", error, error.message());

            None
        }
    }
}

pub fn RegisterClass(className: &str) -> ATOM {
    let instance = GetModuleHandle();
    debug_assert!(instance.0 != 0);

    let wc = WNDCLASSA {
        hCursor: LoadCursor(),
        hInstance: instance,
        lpszClassName: PSTR(className.as_ptr() as _),
        style: CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(wndproc),
        cbClsExtra: 0,
        cbWndExtra: 24,
        hIcon: HICON(0),
        hbrBackground: HBRUSH(5),
        lpszMenuName: PSTR(std::ptr::null_mut()),
        ..Default::default()
    };

    unsafe {
        let res = RegisterClassA(&wc);
        if res == 0 {
            let error = HRESULT(GetLastError() as i32);

            println!("1) Error: {:?}. Message: {:?}", error, error.message());
        }
        res
    }
}

extern "system" fn wndproc(hwnd: HWND, message: u32, wParam: WPARAM, lParam: LPARAM) -> LRESULT {

    unsafe {
        Notifier::shared().try_lock().ok().as_mut().map(|e|e.notify(
            Some(MSG { hwnd, message, wParam, lParam, time: 0, pt: POINT { x:0, y:0 } })
        ));

        match message as u32 {
            WM_PAINT => {
                //println!("WM_PAINT");

                let mut ps = PAINTSTRUCT::default();
                BeginPaint(hwnd, &mut ps);
                // toDO: Draw here
                EndPaint(hwnd, &ps);
            },
            WM_SIZE | WM_SIZING => {
                let rect = GetWindowRect(hwnd);
                AnchorMap::shared().try_lock().as_mut().ok().and_then(|am|am.handleAnchors(rect));
            },
            WM_DESTROY => {
                //println!("WM_DESTROY");
                PostQuitMessage(0);
            },
            _ => {},
        }
        DefWindowProcA(hwnd, message, wParam, lParam)
    }
}

pub fn SetDefaultWindowProc(hwnd: HWND) -> bool {
    extern "system" fn customWinProc(hwnd: HWND, message: u32, wParam: WPARAM, lParam: LPARAM, _uidsubclass: usize, _dwrefdata: usize) -> LRESULT {
        unsafe {
            Notifier::shared().try_lock().ok().as_mut().map(|e|e.notify(
                Some(MSG { hwnd, message, wParam, lParam, time: 0, pt: POINT { x:0, y:0 } })
            ));

            windows::Win32::UI::Shell::DefSubclassProc(hwnd, message, wParam, lParam)
        }
    }

    unsafe {
        windows::Win32::UI::Shell::SetWindowSubclass(hwnd, Some(customWinProc), 0, 0) == BOOL::from(true)
    }
}

pub fn CreateWindowEx(exStyle: WINDOW_EX_STYLE, style: WINDOW_STYLE, className: &str, parent: Option<HWND>, idx: i32, title: &str, x: i32, y: i32, width: i32, height: i32) -> HWND {
    let x = if x > 0  { x } else { CW_USEDEFAULT };
    let y = if y > 0 { y } else { CW_USEDEFAULT };
    let width = if width > 0 { width } else { CW_USEDEFAULT };
    let height = if height > 0 { height } else { CW_USEDEFAULT };

    unsafe {
        let instance = GetModuleHandleA(None);
        let windowClassName = PSTR(className.as_ptr() as _);

        let hwnd = CreateWindowExA(
            exStyle,
            windowClassName,
            title,
            style,
            x,
            y,
            width,
            height,
            parent,
            HMENU(idx as isize),
            instance,
            std::ptr::null_mut(),
        );

        hwnd
    }
}
#[allow(dead_code)]
pub fn GetMessage() -> Option<MSG> {
    let mut message = MSG::default();

    unsafe {

        if GetMessageA(&mut message, HWND(0), 0, 0).into() {
            Some(message)
        } else {
            None
        }
    }
}

pub fn PeekMessage() -> Option<MSG> {
    let mut message = MSG::default();

    unsafe {

        if PeekMessageA(&mut message, HWND(0), 0, 0, PM_REMOVE).into() {
            Some(message)
        } else {
            None
        }
    }
}
#[allow(dead_code)]
pub fn FindWindow(className: &str) -> HWND {
    unsafe {
        FindWindowA(PSTR(className.as_ptr() as _), None)
    }
}

pub fn DispatchMessage(message: *const MSG) -> LRESULT { unsafe { DispatchMessageA(message) } }
pub fn TranslateMessage(message: *const MSG) -> BOOL { unsafe { windows::Win32::UI::WindowsAndMessaging::TranslateMessage(message) } }

#[allow(dead_code)]
pub type WindowCallback = fn(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT;

#[allow(dead_code)]
pub fn SetUserData<T>(hwnd: HWND, data: &T) -> isize {
    unsafe {
        let user_data = data as *const _ as *mut c_void as isize;
        SetWindowLongPtrA(hwnd, GWLP_USERDATA, user_data)
    }
}

pub fn GetWindowText(hwnd: HWND) -> Option<String> {
    unsafe {
        let size = GetWindowTextLengthW(hwnd) as usize;
        let mut text: Vec<u16> = vec![0u16; size + 1];
        let len = GetWindowTextW(hwnd, PWSTR(text.as_mut_ptr()), size as _);
        let text = String::from_utf16_lossy(&text[..len as usize]);

        if text.is_empty() {
            None
        } else {
            Some(text)
        }
    }
}

pub fn GetClassName(hwnd: HWND) -> Option<String> {
    unsafe {
        let size: usize = 512;
        let mut text: Vec<u16> = vec![0; size + 1];
        let len = GetClassNameW(hwnd, PWSTR(text.as_mut_ptr()), size as _);
        let text = String::from_utf16_lossy(&text[..len as usize]);

        if text.is_empty() {
            None
        } else {
            Some(text)
        }
    }
}

pub fn SetWindowText(hwnd: HWND, text: &str) -> BOOL {
    unsafe {
        let res = SetWindowTextA(hwnd, PSTR((format!("{}\0", text)).as_ptr() as _));

        if res != true {
            let error = HRESULT(GetLastError() as i32);

            println!("3) Error: {:?}. Message: {:?}", error, error.message());
        }

        res
    }
}

pub fn CoInitializeEx() -> windows::core::Result<()>{
    unsafe {
        windows::Win32::System::Com::CoInitializeEx(
            std::ptr::null_mut(),
            windows::Win32::System::Com::COINIT_MULTITHREADED
        )
    }
}

pub fn DestroyWindow(hwnd: HWND) -> bool {
    unsafe {
        let res = windows::Win32::UI::WindowsAndMessaging::DestroyWindow(hwnd);

        res.as_bool()
    }
}

static mut GLOBALFONT: Option<windows::Win32::Graphics::Gdi::HFONT> = None;

pub fn GetSystemFont() -> Option<windows::Win32::Graphics::Gdi::HFONT> {
    unsafe {
        if let Some(_) = GLOBALFONT {
            return GLOBALFONT;
        }
        let mut ncm = windows::Win32::UI::WindowsAndMessaging::NONCLIENTMETRICSA::default();
        ncm.cbSize = std::mem::size_of::<NONCLIENTMETRICSA>() as _;
        let res = windows::Win32::UI::WindowsAndMessaging::SystemParametersInfoA(
            SPI_GETNONCLIENTMETRICS,
            std::mem::size_of::<NONCLIENTMETRICSA>() as _,
            &ncm as *const _ as _, 0 as SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS,
        );
        if res == true {
            GLOBALFONT = Some(windows::Win32::Graphics::Gdi::CreateFontIndirectA(&ncm.lfMenuFont));
        }
        GLOBALFONT
    }
}

pub fn GetSystemFontFace() -> String {
    if let Some(hFont) = GetSystemFont() {
        let fontInfo = GetFont(hFont);
        let toString = |a:[CHAR; 32]| String::from_iter(a.iter().filter(|CHAR(a)|*a != 0).map(|CHAR(a)|*a as char));

        if let Some(fontInfo) = fontInfo {
            return toString(fontInfo.lfFaceName);
        }
    }

    return String::from("Segoe UI");
}

pub fn GetFont(hFont: HFONT) -> Option<windows::Win32::Graphics::Gdi::LOGFONTA> {
    let fontInfo = windows::Win32::Graphics::Gdi::LOGFONTA::default();
    unsafe {
        let _res = windows::Win32::Graphics::Gdi::GetObjectA(
            hFont,
            std::mem::size_of::<windows::Win32::Graphics::Gdi::LOGFONTA>() as _,
            &fontInfo as *const _ as _
        );

        Some(fontInfo)
    }
}

pub fn DeleteFont(hFont: HFONT) -> bool {
    unsafe {
        windows::Win32::Graphics::Gdi::DeleteObject(hFont) == true
    }
}
#[allow(dead_code)]
pub fn GetWindowFontFace(hwnd: HWND) -> Result<String, String> {
    let hFont = GetWindowFont(hwnd)?;
    let fontInfo = GetFont(hFont);
    let toString = |a:[CHAR; 32]| String::from_iter(a.iter().filter(|CHAR(a)|*a != 0).map(|CHAR(a)|*a as char));

    if let Some(fontInfo) = fontInfo {
        let fontFace = toString(fontInfo.lfFaceName);
        Ok(fontFace)
    } else {
        Ok(String::from("Segoe UI"))
    }
}

pub fn SetWindowFontFace(hwnd: HWND, fontFace: &str) -> Option<HFONT> {
    let hFont = GetSystemFont()?;
    let mut fontInfo = GetFont(hFont)?;

    let a = fontFace.chars().map(|c|CHAR(c as _)).collect::<Vec<_>>();
    let mut b = [CHAR(0); 32];
    b[..a.len()].copy_from_slice(&a);
    fontInfo.lfFaceName = b;

    unsafe {
        let windowHFont = windows::Win32::Graphics::Gdi::CreateFontIndirectA(&fontInfo);

        let _res = SetWindowFont(hwnd, windowHFont);

        Some(windowHFont)
    }
}

pub fn SetWindowFont(hwnd: HWND, hFont: HFONT) -> LRESULT {
    let HFONT(hFont) = hFont;
    unsafe {
        SendMessageA(hwnd, WM_SETFONT, WPARAM(hFont as _), LPARAM(1))
    }
}
#[allow(dead_code)]
pub fn GetWindowFont(hwnd: HWND) -> Result<HFONT, String> {
    unsafe {
        let LRESULT(hFont) = SendMessageA(hwnd, WM_GETFONT, WPARAM(0), LPARAM(0));
        if hFont == 0 {
            let error = HRESULT(GetLastError() as i32);
            Err(error.message().to_string_lossy())
        } else {
            Ok(HFONT(hFont))
        }
    }
}
#[allow(dead_code)]
pub fn GetControlPosition(hwnd: HWND) -> (i32, i32, i32, i32) {
    unsafe {
        let mut rc = RECT { ..Default::default() };
        //GetClientRect(hwnd, &mut rc);
        windows::Win32::UI::WindowsAndMessaging::GetWindowRect(hwnd, &mut rc);
        let mut leftTop = POINT {
            x: rc.top,
            y: rc.left,
        };
        let parentHwnd = GetParent(hwnd);
        windows::Win32::Graphics::Gdi::MapWindowPoints(hwnd, parentHwnd, &mut leftTop, 2);

        (rc.left, rc.top, rc.right, rc.bottom)
    }
}

pub fn GetClientRect(hwnd: HWND) -> Option<RECT> {
    unsafe {
        let mut rc = RECT { ..Default::default() };
        if windows::Win32::UI::WindowsAndMessaging::GetClientRect(hwnd, &mut rc) == true {
            Some(rc)
        } else {
            None
        }
    }
}

pub fn GetWindowRect(hwnd: HWND) -> Option<RECT> {
    unsafe {
        let mut rc = RECT { ..Default::default() };
        if windows::Win32::UI::WindowsAndMessaging::GetWindowRect(hwnd, &mut rc) == true {
            Some(rc)
        } else {
            None
        }
    }
}
#[allow(dead_code)]
pub fn SetControlPosition(hwnd: HWND, posX: i32, posY: i32, width: i32, height: i32) -> bool {
    unsafe {
        let mut leftTop = POINT {
            x: posX,
            y: posY,
        };
        let HWND(parentHwnd) = GetParent(hwnd);
        windows::Win32::Graphics::Gdi::MapWindowPoints(hwnd, HWND(parentHwnd), &mut leftTop, 2);
        MoveWindow(hwnd, posX, posY, width, height, BOOL::from(true)) == true
    }
}

pub fn GetParent(hwnd: HWND) -> HWND {
    unsafe {
        windows::Win32::UI::WindowsAndMessaging::GetParent(hwnd)
    }
}
#[allow(dead_code)]
pub fn GetControlRect(posX: i32, posY: i32, width: i32, height: i32, style: WINDOW_STYLE) -> (i32, i32, i32, i32) {
    unsafe {
        let mut rc = RECT {
            left: posX,
            top: posY,
            bottom: posX + width,
            right: posY + height,
        };
        AdjustWindowRect(&mut rc, style, BOOL::from(false));

        (rc.left, rc.top, rc.right - rc.left, rc.right - rc.top)
    }
}

pub fn SendMessage(hwnd: HWND, msg: u32, wParam: WPARAM, lParam: LPARAM) -> LRESULT {
    unsafe {
        SendMessageA(hwnd, msg, 
            wParam,
            lParam
        )
    }
}

pub fn ScreenToClient(hwnd: HWND, point: &POINT) -> Option<POINT> {
    let mut pt = point.clone();
    unsafe {
        if windows::Win32::Graphics::Gdi::ScreenToClient(hwnd, &mut pt) == true {
            Some(pt)
        } else {
            None
        }
    }
}

pub fn GetWindowLong(hwnd: HWND, style: WINDOW_LONG_PTR_INDEX) -> i32 {
    unsafe {
        GetWindowLongA(hwnd, style)
    }
}

pub fn GetWindowPlacement(hwnd: HWND) -> Option<WINDOWPLACEMENT> {
    let mut wp = WINDOWPLACEMENT { ..Default::default() };
    unsafe {
        if windows::Win32::UI::WindowsAndMessaging::GetWindowPlacement(hwnd, &mut wp) == true {
            Some(wp)
        } else {
            None
        }
    }
}

pub fn IsWindowVisible(hwnd: HWND) -> bool {
    unsafe {
        windows::Win32::UI::WindowsAndMessaging::IsWindowVisible(hwnd) == true
    }
}

pub fn ShowWindow(hwnd: HWND, cmd: SHOW_WINDOW_CMD) -> bool {
    unsafe {
        windows::Win32::UI::WindowsAndMessaging::ShowWindow(hwnd, cmd) == true
    }
}

pub fn BeginDeferWindowPos(posInfo: i32) -> isize {
    unsafe {
        windows::Win32::UI::WindowsAndMessaging::BeginDeferWindowPos(posInfo)
    }
}

pub fn EndDeferWindowPos(posInfo: isize) -> bool {
    unsafe {
        windows::Win32::UI::WindowsAndMessaging::EndDeferWindowPos(posInfo) == true
    }
}

pub fn DeferWindowPos(posInfo: isize, hwnd: HWND, insertAfter: HWND, posX: i32, posY: i32, width: i32, height: i32, flags: SET_WINDOW_POS_FLAGS) -> isize {
    unsafe {
        windows::Win32::UI::WindowsAndMessaging::DeferWindowPos(
            posInfo, hwnd, insertAfter,
            posX, posY, width, height,
            flags
        )
    }
}

pub fn GetDlgItem(parent: HWND, controlId: i32) -> Option<HWND> {
    unsafe {
        let res = windows::Win32::UI::WindowsAndMessaging::GetDlgItem(parent, controlId);
        if res == HWND(0) {
            None
        } else {
            Some(res)
        }
    }
}

pub fn IsSelected(hwnd: HWND) -> bool {
    unsafe {
        if windows::Win32::UI::WindowsAndMessaging::SendMessageA(hwnd, BM_GETCHECK, WPARAM(0), LPARAM(0)) == LRESULT(0) {
            return false;
        } else {
            return true;
        }
    }
}

pub fn MarkSelected(hwnd: HWND, selected: bool) -> bool {
    let selected = if selected { WPARAM(1) } else { WPARAM(0) };
    unsafe {
        if windows::Win32::UI::WindowsAndMessaging::SendMessageA(hwnd, BM_SETCHECK, selected, LPARAM(0)) == LRESULT(0) {
            return false;
        } else {
            return true;
        }
    }
}

pub fn SelectGetCurrentIndex(hwnd: HWND) -> Option<usize> {
    let className = GetClassName(hwnd)?;
    if className != "ComboBox" {
        return None;
    }
    let LRESULT(index) = SendMessage(hwnd, CB_GETCURSEL, WPARAM(0), LPARAM(0));
    if index == -1 {
        None
    } else {
        Some(index as usize)
    }
}

pub fn SelectGetItemText(hwnd: HWND, index: usize) -> Option<String> {
    let className = GetClassName(hwnd)?;
    if className != "ComboBox" {
        return None;
    }

    unsafe {
        let LRESULT(length) = SendMessageW(hwnd, CB_GETLBTEXTLEN, WPARAM(0), LPARAM(0));
        let size = length as usize;
        let mut buffer: Vec<u16> = vec![0u16; size + 2];
        if size == 0 {
            return None;
        }
        let LRESULT(_length) = SendMessageW(hwnd, CB_GETLBTEXT, WPARAM(index), LPARAM(buffer.as_mut_ptr() as _));
        let text = String::from_utf16_lossy(&buffer[..size]);
        if text.is_empty() {
            return None;
        }

        return Some(text);
    }
}

pub fn SelectSetCurrentIndex(hwnd: HWND, index: usize) -> usize{
    let LRESULT(res) = SendMessage(hwnd, CB_SETCURSEL, WPARAM(index as _), LPARAM(0));
    res as _
}
#[allow(dead_code)]
pub fn SelectAddOption(hwnd: HWND, option: &str) -> bool {
    let lparam = LPARAM(option.as_ptr() as _);
    if SendMessage(hwnd, CB_ADDSTRING, WPARAM(0), lparam) == LRESULT(0) {
        true
    } else {
        false
    }
}

pub fn SelectGetItemsCount(hwnd: HWND) -> isize {
    let LRESULT(count) = SendMessage(hwnd, CB_GETCOUNT, WPARAM(0), LPARAM(0));

    count
}

pub fn SelectSetItems(hwnd: HWND, items: &Vec<String>) {
    SendMessage(hwnd, CB_RESETCONTENT, WPARAM(0), LPARAM(0));
    for item in items {
        let mut text = format!("{}\0", item);
        let lparam = LPARAM(text.as_mut_ptr() as _);

        SendMessage(hwnd, CB_ADDSTRING, WPARAM(0), lparam);
    }
}

pub fn SelectGetItems(hwnd: HWND) -> Vec<String> {
    unsafe {
        let count = SelectGetItemsCount(hwnd) as usize;
        let mut items = vec![];
        for index in 0..count {
            let LRESULT(size) = SendMessageW(hwnd, CB_GETLBTEXTLEN, WPARAM(index as _), LPARAM(0));
            let size = size as usize;
            let mut text: Vec<u16> = vec![0u16; size + 1];
            let LRESULT(_length) = SendMessageW(hwnd, CB_GETLBTEXT, WPARAM(index as _), LPARAM(text.as_mut_ptr() as _));
            let text = String::from_utf16_lossy(&text[..size]);

            items.push(text);
        }

        items
    }
}
