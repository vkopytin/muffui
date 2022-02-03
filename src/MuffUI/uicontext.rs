#![allow(non_snake_case)]
use std::fmt;
use std::rc::Rc;
use std::collections::HashMap;
use std::cmp::Eq;
use std::cmp::PartialEq;
use std::hash::Hash;
use std::hash::Hasher;
use crate::Win;
use crate::muffui::*;


#[derive(Clone)]
pub struct ControlInfo {
    pub hwnd: Win::HWND,
    pub hFont: Option<Win::HFONT>,
    pub isInitialized: isize,
    pub listeners: Vec<SharedProps>,
}

impl fmt::Debug for ControlInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ HWND: {:?}, hFont: {:?} listeners: {:?} }}", self.hwnd, self.hFont, self.listeners)
    }
}

impl Default for ControlInfo {
    fn default() -> Self {
        Self {
            hwnd: Default::default(),
            hFont: Default::default(),
            isInitialized: Default::default(),
            listeners: vec![],
        }
    }
}

impl Hash for ControlInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let Win::HWND(hwnd) = self.hwnd;
        hwnd.hash(state);
    }
}

impl PartialEq for ControlInfo {
    fn eq(&self, other: &Self) -> bool {
        let (Win::HWND(left), Win::HWND(right)) = (self.hwnd, other.hwnd);
        left == right
    }
}

impl Eq for ControlInfo { }

#[derive(Clone)]
pub struct UIContext {
    pub items: HashMap<String, ControlInfo>,
}

impl UIContext {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub fn create() -> Box<Self> {
        Box::new(Self::new())
    }

    pub fn render<T: Renderable>(self: Box<Self>, parent: &str, index: &str, view: &T, msg: Option<Win::MSG>) -> Box<Self> {
        let controlInfo = self.items.get(index);
        let parentHwnd = self.items.get(parent).map(|ci|ci.hwnd);

        if let Some(msg) = msg {
            let props = view.toViewState();
            let listeners = UIContext::collectEvents(controlInfo, msg, props);

            if let Some(ci) = controlInfo {
                return Box::new(Self {
                    items: utils::merge(self.items.clone(), HashMap::from([(String::from(index), ControlInfo {
                        listeners,
                        ..ci.clone()
                    })])),
                });
            } else {
                return self;
            }
        }

        let props = view.toViewState();
        let viewType = props.prop(&SP::Renderer(""));
        if viewType.is_none() {
            return self;
        }

        let controlInfo = self.renderProperties(controlInfo, parentHwnd, props);

        match controlInfo {
            Some(controlInfo) => Box::new(Self {
                items: utils::merge(self.items.clone(), HashMap::from([(String::from(index), controlInfo)])),
                ..*self
            }),
            _ => self,
        }
    }

    fn collectEvents(_controlInfo: Option<&ControlInfo>, _msg: Win::MSG, props: Vec<SharedProps>) -> Vec<SharedProps> {
        let mut listeners = vec![];

        for prop in props.into_iter() {
            let isListener = match prop {
                SharedProps::DidResize(_)
                | SharedProps::DidClick(_)
                | SharedProps::DidChange(_) => true,
                _ => false,
            };

            if isListener {
                listeners.push(prop);
            }
        }

        listeners
    }

    fn renderProperties(&self, controlInfo: Option<&ControlInfo>, parent: Option<Win::HWND>, props: Vec<SharedProps>) -> Option<ControlInfo> {
        let mut hFont = None;
        let mut hwnd = None;
        let mut isInitialized = 1;
        let mut listeners = vec![];
        if let Some(controlInfo) = controlInfo {
            hFont = controlInfo.hFont;
            hwnd = Some(controlInfo.hwnd);
            isInitialized = controlInfo.isInitialized;
            isInitialized += 1;
            listeners = controlInfo.listeners.iter().map(|a|a.clone()).collect();
        }
        let mut fontFace = Rc::from(Win::GetSystemFontFace().as_str());
        let mut className = Rc::from("");
        let mut title = Rc::from("");
        let mut renderer = Rc::from("");
        let mut posX = Win::CW_USEDEFAULT;
        let mut posY = Win::CW_USEDEFAULT;
        let mut height = Win::CW_USEDEFAULT;
        let mut width = Win::CW_USEDEFAULT;
        let mut idx = 0;
        let mut dock = 0;
        let mut isSelected = false;
        let mut selectItems = vec![];
        let mut selectedIndex = 0;
        for prop in props.into_iter() {
            match prop {
                SharedProps::ClassName(v) => className = v,
                SharedProps::Title(v) => title = v,
                SharedProps::Renderer(v) => renderer = v,
                SharedProps::PosX(x) => posX = x,
                SharedProps::PosY(y) => posY = y,
                SharedProps::Width(w) => width = w,
                SharedProps::Height(h) => height = h,
                SharedProps::ControlId(i) => idx = i,
                SharedProps::FontFace(f) => fontFace = f,
                SharedProps::Anchor(d) => dock = d,
                SharedProps::Selected(s) => isSelected = s,
                SharedProps::SelectItems(i) => selectItems = i,
                SharedProps::SelectedIndex(i) => selectedIndex = i,
                _ => (),
            };
        }

        let renderer: &str = &renderer;
        let style = match renderer {
            "window" => Win::WS_OVERLAPPEDWINDOW | Win::WS_VISIBLE,
            "panel" => Win::WS_TABSTOP | Win::WS_CHILD | Win::WS_VISIBLE | Win::WS_CLIPSIBLINGS | Win::WS_BORDER,
            "check-box" => (Win::BS_CHECKBOX as Win::WINDOW_STYLE) | Win::WS_TABSTOP | Win::WS_CHILD | Win::WS_VISIBLE,
            "group-box" => (Win::BS_GROUPBOX as Win::WINDOW_STYLE) | Win::WS_TABSTOP | Win::WS_CHILD | Win::WS_VISIBLE,
            "text-box" => Win::WS_CHILD | Win::WS_VISIBLE,
            "label" => Win::WS_CHILD | Win::WS_VISIBLE,
            "button" => Win::WS_TABSTOP | Win::WS_BORDER | Win::WS_CHILD | Win::WS_VISIBLE,
            "radio-box" => (Win::BS_RADIOBUTTON as Win::WINDOW_STYLE) | Win::WS_TABSTOP | Win::WS_CHILD | Win::WS_VISIBLE,
            "select0" => (Win::CBS_HASSTRINGS as Win::WINDOW_STYLE) | Win::WS_CHILD | Win::WS_VISIBLE | Win::WS_VSCROLL,
            "select" => ((Win::CBS_DROPDOWN | Win::CBS_HASSTRINGS | Win::CBS_AUTOHSCROLL) as Win::WINDOW_STYLE) | Win::WS_CHILD | Win::WS_VISIBLE | Win::WS_VSCROLL,
            "select2" => ((Win::CBS_DROPDOWNLIST | Win::BS_DEFSPLITBUTTON | Win::CBS_DROPDOWN | Win::CBS_HASSTRINGS) as Win::WINDOW_STYLE) | Win::WS_VISIBLE | Win::WS_CHILD | Win::WS_VSCROLL,  // Styles WS_VSCROLL | BS_DEFSPLITBUTTON WS_DISABLED | 
            _ => Win::WS_BORDER,
        };

        let exStyle: Win::WINDOW_EX_STYLE = match renderer {
            "group-box" => Win::WS_EX_CONTROLPARENT,
            //"panel" => Win::WS_EX_CLIENTEDGE,
            "text-box" => Win::WS_EX_CLIENTEDGE,
            _ => Default::default(),
        };

        let res = Win::GetClassInfoEx(format!("{}\0", className).as_str());
        if res == None {
            let atom = Win::RegisterClass(format!("{}\0", className).as_str());
            debug_assert!(atom != 0);
        }

        let mut hwnd = match hwnd {
            Some(hwnd) => hwnd,
            _ => {
                let hwnd = Win::CreateWindowEx(
                    exStyle, style, format!("{}\0", className).as_str(), parent, idx, &title,
                    posX, posY, width, height
                );
                if &*renderer == "group-box" {
                    Win::SetDefaultWindowProc(hwnd);
                }
                AnchorMap::shared().lock().as_mut().ok().and_then(|am|{
                    am.addControl(idx, dock, Some(hwnd));
                    let rect = Win::GetWindowRect(am.parent);
                    am.handleAnchors(rect)
                });
                
                hwnd
            }
        };

        if let Some(oldClassName) = Win::GetClassName(hwnd) {
            if oldClassName != &*className {
                if Win::DestroyWindow(hwnd) {
                    hwnd = Win::CreateWindowEx(exStyle, style, format!("{}\0", className).as_str(), parent, idx, &title, posX, posY, width, height);
                }
            }
        }

        let toString = |a:[Win::CHAR; 32]| String::from_iter(a.iter().filter(|Win::CHAR(a)|*a != 0).map(|Win::CHAR(a)|*a as char));
        hFont = hFont.and_then(Win::GetFont).map(|f|f.lfFaceName).map(toString)
            .filter(|f|f == &*fontFace).and(hFont)
            .or_else(||{
                Win::DeleteFont(hFont?);
                None
            });
        hFont = hFont.or_else(||Win::SetWindowFontFace(hwnd, &fontFace));

        if let Some(oldTitle) = Win::GetWindowText(hwnd) {
            if oldTitle != &*title {
                Win::SetWindowText(hwnd, &title);
            }
        }

        if isSelected != Win::IsSelected(hwnd) {
            Win::MarkSelected(hwnd, isSelected);
        }

        let oldSelectItems = Win::SelectGetItems(hwnd);
        if oldSelectItems != selectItems {
            Win::SelectSetItems(hwnd, &selectItems);
        }

        let oldSelectedIndex = Win::SelectGetCurrentIndex(hwnd);
        if let Some(oldSelectedIndex) = oldSelectedIndex {
            if selectedIndex != oldSelectedIndex {
                Win::SelectSetCurrentIndex(hwnd, selectedIndex);
            } else if let Some(currentText) = Win::SelectGetItemText(hwnd, selectedIndex) {
                if &*title != currentText {
                    Win::SelectSetCurrentIndex(hwnd, selectedIndex);
                }
            }
        }

        if "window" == renderer && isInitialized == 2 {
            AnchorMap::shared().lock().as_mut().ok().and_then(|am|am.initialize(
                hwnd, ANF_TOP | ANF_LEFT | ANF_RIGHT
            ));
            let rect = Win::GetWindowRect(hwnd);
            AnchorMap::shared().lock().as_mut().ok().and_then(|am|am.handleAnchors(rect));
            isInitialized += 1;
        } else if isInitialized > 2 {
            isInitialized = 2;
        }

        Some(ControlInfo { hwnd, hFont, isInitialized, listeners })
    }
}
