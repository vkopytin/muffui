#![allow(non_snake_case)]

use std::sync::Once;
use std::mem::MaybeUninit;
use std::sync::Mutex;

macro_rules! if_then {
    ($c:expr, $v:expr) => {
        if $c {$v}
    };
}

use crate::Win;

/// proportinally moves the control with the size of the window
pub const ANF_NONE: usize = 0x0000;
/// docks the control to the top of the window
pub const ANF_DOCK_TOP: usize =  0x0001;
/// docks the control to the bottom of the window
pub const ANF_DOCK_BOTTOM: usize =  0x0002;
/// docks the control to the left of the window
pub const ANF_DOCK_LEFT: usize =  0x0004;
/// docks the control to the right of the window
pub const ANF_DOCK_RIGHT: usize =  0x0008;
/// docks the control to all sides of the window
pub const ANF_DOCK_ALL: usize =  0x000F;
/// distance of the control to the top of the window will be constant
pub const ANF_TOP: usize =  0x0010;
/// distance of the control to the bottom of the window will be constant
pub const ANF_BOTTOM: usize =  0x0020;
/// distance of the control to the left of the window will be constant
pub const ANF_LEFT: usize =  0x0040;   
/// distance of the control to the right of the window will be constant
pub const ANF_RIGHT: usize =  0x0080;
/// automatically calculate the anchors, cannot be used with other flags
pub const ANF_AUTOMATIC: usize =  0x0100;
/// docks the top of the control to the top of the window
pub const ANF_DOCK_TOP_EX: usize =  0x0200;
/// docks the bottom of the control to the bottom of the window
pub const ANF_DOCK_BOTTOM_EX: usize =  0x0400;
/// docks the left-side of the control to the left-side of the window
pub const ANF_DOCK_LEFT_EX: usize =  0x0800;
/// docks the right-side of the control to the right-side of the window
pub const ANF_DOCK_RIGHT_EX: usize =  0x1000;
/// some additional control flags
///
/// forces to erase the background of the control in EraseBackground
#[allow(dead_code)]
pub const ANF_ERASE: usize =  0x2000;

/// some combinations
#[allow(dead_code)]
pub const ANF_TOPLEFT: usize = ANF_TOP | ANF_LEFT;   
/// some combinations
#[allow(dead_code)]
pub const ANF_TOPRIGHT: usize = ANF_TOP | ANF_RIGHT;
/// some combinations
#[allow(dead_code)]
pub const ANF_BOTTOMLEFT: usize = ANF_BOTTOM | ANF_LEFT;
/// some combinations
#[allow(dead_code)]
pub const ANF_BOTTOMRIGHT: usize = ANF_BOTTOM | ANF_RIGHT;
/// some combinations
pub const ANF_TOPBOTTOM: usize = ANF_TOP | ANF_BOTTOM;
/// some combinations
pub const ANF_LEFTRIGHT: usize = ANF_LEFT | ANF_RIGHT;

/// flags for InitAnchors 
///             
/// calculate size occupied by all controls, useful for formviews       
pub const ANIF_CALCSIZE: usize =  0x0001;
/// flags for InitAnchors  
///            
/// add a sizing-grip to the parent window
pub const ANIF_SIZEGRIP: usize =  0x0002;

struct FRECT {
    top: f32,
    left: f32,
    right: f32,
    bottom: f32,
}

impl FRECT {
    pub fn set(&mut self, left: f32, top: f32, right: f32, bottom: f32) {
        self.left = left;
        self.top = top;
        self.right = right;
        self.bottom = bottom;
    }
}

impl Default for FRECT {
    fn default() -> Self {
        Self {
            top: 0f32,
            left: 0f32,
            right: 0f32,
            bottom: 0f32,
        }
    }
}

struct FSIZE {
    cx: f32,
    cy: f32,
}

pub struct ControlEntry {
    controlId: i32,
    flags: usize,
    rect: FRECT,
    hwnd: Win::HWND,
}

pub struct AnchorMap {
    prev: Win::RECT,
    isInitialized: bool,
    current: Win::RECT,
    client: Win::RECT,
    sizedBorders: usize,
    delta: Win::SIZE,
    parent: Win::HWND,
    count: i32,
    controls: Vec<ControlEntry>,
    defaultEntry: bool,
    defaultFlags: usize,
    #[allow(dead_code)]
    backgroundColor: u32,
    sizeGrip: Win::HWND,
}

impl Default for AnchorMap {
    fn default() -> Self {
        Self {
            prev: Default::default(),
            isInitialized: false,
            current: Default::default(),
            client: Default::default(),
            sizedBorders: 0,
            delta: Default::default(),
            parent: Win::HWND(0),
            controls: vec![],
            count: 0,
            defaultEntry: false,
            defaultFlags: 0,
            backgroundColor: unsafe { Win::GetSysColor(Win::COLOR_BTNFACE) },
            sizeGrip: Default::default(),
        }
    }
}

impl AnchorMap {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn shared() -> &'static Mutex<AnchorMap> {
        static mut CONF: MaybeUninit<Mutex<AnchorMap>> = MaybeUninit::uninit();
        static ONCE: Once = Once::new();
    
        ONCE.call_once(|| unsafe {
            CONF.as_mut_ptr().write(Mutex::new(AnchorMap::new()));
        });

        unsafe { &*CONF.as_ptr() }
    }

    pub fn addControl(&mut self, controlId: i32, flags: usize, hwnd: Option<Win::HWND>) -> bool {
        if controlId == 0 {
            self.defaultEntry = true;
            self.defaultFlags = flags;
        } else {
            let entry = ControlEntry {
                controlId,
                flags,
                rect: FRECT { left: 0f32, top: 0f32, right: 0f32, bottom: 0f32 },
                hwnd: hwnd.unwrap_or(Win::HWND(0)),
            };
            self.controls.push(entry);

            self.count = self.controls.len() as i32;
        }

        true
    }

    pub fn isInitialized(&self) -> bool {
        self.isInitialized
    }

    #[allow(dead_code)]
    pub fn findWindow(&self, hwnd: Win::HWND) -> Option<&ControlEntry> {
        self.controls.iter().find(|c|c.hwnd == hwnd)
    }

    pub fn ScreenToClient(hwnd: Win::HWND, rect: &Win::RECT) -> Option<Win::RECT> {
        let mut pt1 = Win::POINT { ..Default::default() };
        let mut pt2 = Win::POINT { ..Default::default() };
        pt1.x = rect.left;
        pt1.y = rect.top;
        pt2.x = rect.right;
        pt2.y = rect.bottom;

        let pt1 = Win::ScreenToClient(hwnd, &pt1)?;
        let pt2 = Win::ScreenToClient(hwnd, &pt2)?;

        Some(Win::RECT {
            left: pt1.x,
            top: pt1.y,
            right: pt2.x,
            bottom: pt2.y,
        })
    }

    pub fn preProcess(&mut self, rect: &Win::RECT) {
        self.current.left = rect.left;
        self.current.top = rect.top;
        self.current.right = rect.right;
        self.current.bottom = rect.bottom;

        self.sizedBorders = 0;

        if_then!(self.current.left != self.prev.left, self.sizedBorders |= ANF_LEFT);
        if_then!(self.current.top != self.prev.top, self.sizedBorders |= ANF_TOP);
        if_then!(self.current.right != self.prev.right, self.sizedBorders |= ANF_RIGHT);
        if_then!(self.current.bottom != self.prev.bottom, self.sizedBorders |= ANF_BOTTOM);

        self.delta.cx = (self.current.right - self.current.left) - (self.prev.right - self.prev.left);
        self.delta.cy = (self.current.bottom - self.current.top) - (self.prev.bottom - self.prev.top);

        self.client.right += self.delta.cx;
        self.client.bottom += self.delta.cy;
    }

    pub fn postProcess(&mut self) {
        self.prev.left = self.current.left;
        self.prev.top = self.current.top;
        self.prev.right = self.current.right;
        self.prev.bottom = self.current.bottom;
    }

    pub fn initialize(&mut self, parent: Win::HWND, flags: usize) -> Option<()> {
        self.parent = parent;
        self.prev = Win::GetWindowRect(self.parent)?;
        self.current = self.prev.clone();
        self.client = Win::GetClientRect(self.parent)?;

        for item in self.controls.iter_mut() {
            let hwndControl = Win::GetDlgItem(self.parent, item.controlId)
                .unwrap_or(item.hwnd);

            if Win::HWND(0) != hwndControl {
                item.hwnd = hwndControl;
                let rect = Win::GetWindowRect(hwndControl)?;
                let parent = Win::GetParent(hwndControl);
                let rect = AnchorMap::ScreenToClient(parent, &rect)?;

                item.rect.left = rect.left as _;
                item.rect.top = rect.top as _;
                item.rect.right = rect.right as _;
                item.rect.bottom = rect.bottom as _;
            }
        }

        if self.defaultEntry {
            unsafe {
                let lparam = self as *mut _ as *mut core::ffi::c_void;
                Win::EnumChildWindows(parent, Some(InitDefaultControls), Win::LPARAM(lparam as isize));
                //for (_, ci) in windows.clone().iter() {
                //    println!("Initializeing resizing");
                //    InitDefaultControls(ci.hwnd, Win::LPARAM(lparam as isize));
                //}
            }
        }

        let mut sz1 = Win::SIZE { ..Default::default() };
        if flags & ANIF_SIZEGRIP > 0 {
            let dw1 = self.defaultFlags;
            self.defaultFlags = ANF_RIGHT | ANF_BOTTOM;
            unsafe {
                sz1.cx = Win::GetSystemMetrics(Win::SM_CXVSCROLL);
                sz1.cy = Win::GetSystemMetrics(Win::SM_CYHSCROLL);
            }
            self.sizeGrip = Win::CreateWindowEx(
                Default::default(),
                Win::WS_CHILD | Win::WS_VISIBLE | (Win::SBS_SIZEGRIP as Win::WINDOW_STYLE),
                "ScrollBar", Some(parent), 0, "",
                self.client.right - sz1.cx, self.client.bottom - sz1.cy, sz1.cx, sz1.cy
            );
            unsafe {
                Win::SetWindowPos(
                    self.sizeGrip, Win::HWND_TOP,
                    0,0,0,0,
                    Win::SWP_NOMOVE | Win::SWP_NOACTIVATE | Win::SWP_NOSIZE
                );
            }
            let lparam = self as *mut _ as *mut core::ffi::c_void;
            InitDefaultControls(self.sizeGrip, Win::LPARAM(lparam as isize));
            self.defaultFlags = dw1;
        }

        if flags & ANIF_CALCSIZE > 0 {
            let mut max = Win::RECT { ..Default::default() };
            for item in self.controls.iter() {
                if_then!(item.rect.right > max.right as _, max.right = item.rect.right as _);
                if_then!(item.rect.bottom > max.bottom as _, max.bottom = item.rect.bottom as _);
            }
            self.prev.right = self.prev.left + max.right;
            self.prev.bottom = self.prev.top + max.bottom;
            unsafe {
                let style = Win::GetWindowLong(parent, Win::GWL_STYLE);
                let menu = Win::GetMenu(parent);
                Win::AdjustWindowRect(&mut self.prev, style as Win::WINDOW_STYLE, menu != Win::HMENU(0));
            }
            self.client.right = self.client.left + (self.prev.right - self.prev.left);
            self.client.bottom = self.client.top + (self.prev.bottom - self.prev.top);
        }

        for item in self.controls.iter_mut() {
            let mut client = Win::SIZE { ..Default::default() };
            client.cx = self.client.right - self.client.left;
            client.cy = self.client.bottom - self.client.top;

            if item.flags == ANF_AUTOMATIC {
                item.flags = 0;

                if_then!(item.rect.top < (client.cy / 2) as _, item.flags |= ANF_TOP);
                if_then!(item.rect.bottom >= (client.cy / 2) as _, item.flags |= ANF_BOTTOM);

                if_then!(item.rect.left < (client.cx / 2) as _, item.flags |= ANF_LEFT);
                if_then!(item.rect.right >= (client.cx / 2) as _, item.flags |= ANF_RIGHT);
            }
        }

        self.isInitialized = true;

        Some(())
    }

    pub fn handleAnchors(&mut self, parentRect: Option<Win::RECT>) -> Option<()> {
        if !self.isInitialized() {
            return Some(());
        }

        parentRect
            .or_else(||Win::GetWindowRect(self.parent))
            .map(|pr|self.preProcess(&pr));

        if self.sizeGrip != Win::HWND(0) {
            if let Some(wp) = Win::GetWindowPlacement(self.parent) {
                if wp.showCmd == Win::SW_MAXIMIZE && Win::IsWindowVisible(self.sizeGrip) {
                    Win::ShowWindow(self.sizeGrip, Win::SW_HIDE);
                } else if Win::IsWindowVisible(self.sizeGrip) {
                    Win::ShowWindow(self.sizeGrip, Win::SW_SHOW);
                }
            }
        }

        let mut szControl = FSIZE { cx: 0f32, cy: 0f32 };
        for item in self.controls.iter_mut() {
            let mut isChanged = false;
            if item.hwnd == Win::HWND(0) {
                continue;
            }

            szControl.cx = item.rect.right - item.rect.left;
            szControl.cy = item.rect.bottom - item.rect.top;

            if (item.flags & ANF_DOCK_ALL) == ANF_DOCK_ALL {
                item.rect.set(0f32, 0f32, self.client.right as _, self.client.bottom as _);
                isChanged = true;
            } else if item.flags & ANF_DOCK_TOP > 0 {
                item.rect.set(0f32, 0f32, self.client.right as _, szControl.cy);
                isChanged = true;
            } else if item.flags & ANF_DOCK_BOTTOM > 0 {
                item.rect.set(0f32, self.client.bottom as f32 - szControl.cy, self.client.right as _, self.client.bottom as _);
                isChanged = true;
            } else if item.flags & ANF_DOCK_LEFT > 0 {
                item.rect.set(0f32, 0f32, szControl.cx, self.client.bottom as _);
                isChanged = true;
            } else if item.flags & ANF_DOCK_RIGHT > 0 {
                item.rect.set(self.client.right as f32 - szControl.cx, 0f32, self.client.right as _, self.client.bottom as _);
                isChanged = true;
            } else if item.flags & ANF_DOCK_LEFT_EX > 0 {
                item.rect.set(0f32, item.rect.top, szControl.cx, item.rect.bottom);
                isChanged = true;
            } else if item.flags & ANF_DOCK_RIGHT_EX > 0 {
                item.rect.set(item.rect.left, item.rect.top, self.client.right as _, item.rect.bottom as _);
                isChanged = true;
            } else if item.flags & ANF_DOCK_TOP_EX > 0 {
                item.rect.set(item.rect.left, 0f32, item.rect.right, item.rect.bottom);
                isChanged = true;
            } else if item.flags & ANF_DOCK_BOTTOM_EX > 0 {
                item.rect.set(item.rect.left, item.rect.top, item.rect.right, self.client.bottom as _);
                isChanged = true;
            }

            if (self.sizedBorders & ANF_LEFTRIGHT) > 0 && self.delta.cx != 0 && !isChanged {
                match item.flags & ANF_LEFTRIGHT {
                    ANF_LEFT => {},
                    ANF_RIGHT => {
                        item.rect.left += self.delta.cx as f32;
                        item.rect.right = item.rect.left + szControl.cx;
                        isChanged = true;
                    },
                    ANF_LEFTRIGHT => {
                        item.rect.right += self.delta.cx as f32;
                        isChanged = true;
                    },
                    _ => {
                        item.rect.left += (self.delta.cx / 2) as f32;
                        item.rect.right = item.rect.left + szControl.cx;
                        isChanged = true;
                    },
                };
            }

            if self.sizedBorders & ANF_TOPBOTTOM > 0 && self.delta.cy != 0 {
                match item.flags & ANF_TOPBOTTOM {
                    ANF_TOP => {},
                    ANF_BOTTOM => {
                        item.rect.top += self.delta.cy as f32;
                        item.rect.bottom = item.rect.top + szControl.cy;
                        isChanged = true;
                    },
                    ANF_TOPBOTTOM => {
                        item.rect.bottom += self.delta.cy as f32;
                        isChanged = true;
                    },
                    _ => {
                        item.rect.top += (self.delta.cy / 2) as f32;
                        item.rect.bottom = item.rect.top + szControl.cy;
                        isChanged = true;
                    },
                };
            }

            if isChanged {
                szControl.cx = item.rect.right - item.rect.left;
                szControl.cy = item.rect.bottom - item.rect.top;
                let posInfo = Win::BeginDeferWindowPos(self.count);
                Win::DeferWindowPos(
                    posInfo, item.hwnd, Win::HWND(0),
                    item.rect.left as _, item.rect.top as _, szControl.cx as _, szControl.cy as _,
                    Win::SWP_NOZORDER | Win::SWP_NOOWNERZORDER | Win::SWP_SHOWWINDOW
                );

                Win::EndDeferWindowPos(posInfo);
            }
        }

        self.postProcess();

        Some(())
    }
}

extern "system" fn InitDefaultControls(hwnd: Win::HWND, lParam: Win::LPARAM) -> Win::BOOL {
    let mut anchorMap: &mut AnchorMap = unsafe {
        let Win::LPARAM(lParam) = lParam;
        let closurePointer = lParam as *mut core::ffi::c_void;
        &mut *(closurePointer as *mut _)
    };

    let parentHwnd = Win::GetParent(hwnd);
    if parentHwnd != anchorMap.parent {
        return Win::BOOL::from(true);
    }

    for item in anchorMap.controls.iter() {
        if item.hwnd == hwnd {
            return Win::BOOL::from(true);
        }
    }

    let mut count = anchorMap.count;
    let parent = anchorMap.parent;
    let defaultFlags = anchorMap.defaultFlags;
    if let Some(mut entry) = anchorMap.controls.get_mut(count as usize) {
        entry.hwnd = hwnd;
        unsafe {
            entry.controlId = Win::GetDlgCtrlID(hwnd);
        }
        entry.flags = defaultFlags;
        let rect = Win::GetWindowRect(hwnd)
            .and_then(|rect|AnchorMap::ScreenToClient(parent, &rect))
            ;

        if let Some(rect) = rect {
            entry.rect.top = rect.top as _;
            entry.rect.left = rect.left as _;
            entry.rect.right = rect.right as _;
            entry.rect.bottom = rect.bottom as _;

            count += 1;
        }
    }
    anchorMap.count = count;

    return Win::BOOL::from(true);
}
