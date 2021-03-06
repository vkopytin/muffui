#![allow(non_snake_case)]
use std::fmt;
use std::sync::Arc;
use std::sync::Once;
use std::mem::MaybeUninit;
use std::sync::Mutex;
use crate::muffui::*;
use crate::Win;

#[derive(Clone)]
pub struct EventInfo {
    pub hwnd: Win::HWND,
    pub parent: Win::HWND,
    pub listeners: Vec<SharedProps>,
    pub props: Vec<SharedProps>,
    pub target: Win::HWND,
}

impl fmt::Debug for EventInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ HWND: {:?}, listeners: {:?}, props: {:?} }}", self.hwnd, self.listeners, self.props)
    }
}

impl From<Win::MSG> for EventInfo {
    fn from(msg: Win::MSG) -> Self {
        //let className = Win::GetClassName(msg.hwnd);
        let empty = Self {
            hwnd: Win::HWND(0),
            parent: Win::HWND(0),
            listeners: vec![],
            props: vec![],
            target: Win::HWND(0),
        };
        let defaultEvent = Self {
            hwnd: msg.hwnd,
            parent: Win::HWND(0),
            listeners: vec![],
            props: vec![],
            target: msg.hwnd,
        };
        match msg.message {
            Win::WM_CREATE => {
                let sp = SharedProps::DidCreate(Arc::from(Mutex::from(Command::new(|_|{}))));
                Self {
                    listeners: vec![sp],
                    ..defaultEvent
                }
            },
            Win::WM_SIZE | Win::WM_SIZING => {
                let sp = SharedProps::DidResize(Arc::from(Mutex::from(Command::new(|_|{}))));
                Self {
                    listeners: vec![sp],
                    ..defaultEvent
                }
            },
            Win::WM_CTLCOLORLISTBOX => {
                //println!("WM_CTLCOLORLISTBOX");
                empty
            },
            Win::WM_CTLCOLOREDIT => {
                //println!("WM_CTLCOLOREDIT");
                empty
            },
            Win::WM_MOUSEMOVE => {
                empty
            },
            Win::WM_COMMAND => {
                //println!("WM_COMMAND");
                let Win::WPARAM(wParam) = msg.wParam;
                let Win::LPARAM(control) = msg.lParam;
                if Win::HIWORD(wParam) == Win::CBN_SELCHANGE as _ {
                    let sp = SharedProps::DidChange(Arc::from(Mutex::from(Command::new(|_|{}))));
                    Self {
                        hwnd: Win::HWND(control),
                        listeners: vec![sp],
                        ..defaultEvent
                    }
                } else {
                    empty
                }
            },
            Win::WM_DISPLAYCHANGE => {
                //println!("WM_DISPLAYCHANGE");
                empty
            },
            Win::WM_CHAR => {
                let sp = SharedProps::DidChange(Arc::from(Mutex::from(Command::new(|_|{}))));
                Self {
                    listeners: vec![sp],
                    ..defaultEvent
                }
            },
            Win::WM_SYSKEYUP | Win::WM_KEYUP => {
                let Win::WPARAM(actionId) = msg.wParam;
                let sp = SharedProps::DidClick(Arc::from(Mutex::from(Command::new(|_|{}))));
                match actionId {
                    13 | 32 => Self {
                        listeners: vec![sp],
                        ..defaultEvent
                    },
                    _ => empty,
                }
            },
            Win::WM_NCACTIVATE => {
                empty
            },
            Win::WM_NCHITTEST => {
                empty
            },
            Win::WM_SETCURSOR => {
                let listeners = vec![];
                Self {
                    listeners,
                    ..defaultEvent
                }
            },
            Win::BM_CLICK => {
                let sp = SharedProps::DidClick(Arc::from(Mutex::from(Command::new(|_|{}))));
                let listeners = vec![sp];
                Self {
                    listeners,
                    ..defaultEvent
                }
            },
            Win::WM_LBUTTONUP | Win::WM_RBUTTONUP | Win::WM_MBUTTONUP => {
                let sp = SharedProps::DidClick(Arc::from(Mutex::from(Command::new(|_|{}))));
                let listeners = vec![sp];
                Self {
                    listeners,
                    ..defaultEvent
                }  
            },
            Win::WM_DESTROY => {
                let sp = SharedProps::DidDestroy(Arc::from(Mutex::from(Command::new(|_|{}))));
                let listeners = vec![sp];
                Self {
                    listeners,
                    ..defaultEvent
                }    
            },
            _ => {
                //println!("MSG: {:?}", msg);
                empty
            },
        }
    }
}

pub struct EventHub {
    pub events: Vec<EventInfo>,
}

impl From<EventInfo> for Vec<EventInfo> {
    fn from(value: EventInfo) -> Vec<EventInfo> {
        vec![value]
    }
}

impl Default for EventHub {
    fn default() -> Self {
        Self {
            events: vec![],
        }
    }
}

impl EventHub {
    pub fn new() -> Self {
        Self { ..Default::default() }
    }

    pub fn shared() -> &'static Mutex<EventHub> {
        static mut CONF: MaybeUninit<Mutex<EventHub>> = MaybeUninit::uninit();
        static ONCE: Once = Once::new();
    
        ONCE.call_once(|| unsafe {
            CONF.as_mut_ptr().write(Mutex::new(EventHub::new()));
        });

        unsafe { &*CONF.as_ptr() }
    }

    pub fn enqueueEvent(&mut self, msg: Win::MSG) {
        let event = EventInfo::from(msg);
        if event.listeners.len() == 0 {
            return;
        }
        let mut currentHwnd = event.hwnd;
        while currentHwnd != Win::HWND(0) {
            let parent = Win::GetParent(currentHwnd);
            // Create events with bubble events (For Select/CompoBox edit control)
            if let Some(e) = self.events.iter_mut().find(|e|e.hwnd == currentHwnd) {
                let props = e.props.clone();
                let listeners = e.listeners.clone();
                e.listeners = listeners.merge(event.listeners.clone());
                e.props = props.merge(event.props.clone());
                e.parent = parent;
                e.hwnd = currentHwnd;
            } else {
                let copyEvent = EventInfo {
                    hwnd: currentHwnd,
                    parent,
                    ..event.clone()
                };
                self.events.push(copyEvent);
            }
            currentHwnd = parent;
        }
    }

    pub fn putListener<V: Into<Vec<SharedProps>>>(&mut self, hwnd: Win::HWND, listeners: V) {
        for listener in listeners.into() {
            let isListener = match listener {
                SharedProps::DidResize(_)
                | SharedProps::DidClick(_)
                | SharedProps::DidChange(_)
                | SharedProps::DidDestroy(_)
                | SharedProps::DidCreate(_) => true,
                _ => false,
            };
            if !isListener {
                return;
            }
            if let Some(e) = self.events.iter_mut().find(|e|e.hwnd == hwnd) {
                let clonned = e.listeners.clone();
                e.listeners = clonned.update(listener.clone());
            }
        }
    }

    pub fn dispatchEvents(&mut self) -> bool {
        let mut res = false;
        for e in self.events.iter_mut() {
            let clonned = e.listeners.clone();
            e.listeners = vec![];
            let mut props = vec![];
            let className = Win::GetClassName(e.hwnd);
            if let Some(cn) = className {
                props.push(SP::ClassName(&cn));
            }
            let newTitle = Win::GetWindowText(e.hwnd);
            props.push(SP::Title(&newTitle));

            for l in clonned.iter() {
                let mut props = props.clone();
                res = match l {
                    SharedProps::DidCreate(h) => {
                        let rect = Win::GetWindowRect(e.hwnd)
                            .and_then(|r|AnchorMap::ScreenToClient(e.parent, &r))
                            .unwrap_or(Win::RECT { ..Default::default() });
                        props.push(SharedProps::PosX(rect.left));
                        props.push(SharedProps::PosY(rect.top));
                        props.push(SharedProps::Width(rect.right - rect.left));
                        props.push(SharedProps::Height(rect.bottom - rect.top));

                        let mut h = h.lock().unwrap();
                        h.exec(props);
                        true
                    },
                    SharedProps::DidResize(h) => {
                        let rect = Win::GetWindowRect(e.hwnd)
                            .and_then(|r|AnchorMap::ScreenToClient(e.parent, &r))
                            .unwrap_or(Win::RECT { ..Default::default() });
                        props.push(SharedProps::PosX(rect.left));
                        props.push(SharedProps::PosY(rect.top));
                        props.push(SharedProps::Width(rect.right - rect.left));
                        props.push(SharedProps::Height(rect.bottom - rect.top));

                        let mut h = h.lock().unwrap();
                        h.exec(props);
                        true
                    },
                    SharedProps::DidChange(h) => {
                        if let Some(selectedItem) = Win::SelectGetCurrentIndex(e.hwnd) {
                            // toDO: Get selected title
                            props.push(SharedProps::SelectedIndex(selectedItem));
                        }
                        let mut h = h.lock().unwrap();
                        h.exec(props);
                        true
                    },
                    SharedProps::DidClick(h) => {
                        let mut h = h.lock().unwrap();
                        let selected = Win::IsSelected(e.hwnd);
                        props.push(SharedProps::Selected(selected));
                        h.exec(props);
                        true
                    },
                    SharedProps::DidDestroy(h) => {
                        let mut h = h.lock().unwrap();
                        h.exec(props);
                        true
                    },
                    _ => false
                }
            }
        }
        res
    }
}

pub struct Notifier<E = Option<EventInfo>> {
    subscribers: Vec<Box<dyn FnMut(&E)>>,
}

impl<E> Notifier<E> {
    pub fn new() -> Notifier<E> {
        Notifier {
            subscribers: Vec::new(),
        }
    }

    pub fn register<F>(&mut self, callback: F) where F: 'static + FnMut(&E),
    {
        self.subscribers.push(Box::new(callback));
    }

    pub fn notify(&mut self, event: E) {
        for callback in self.subscribers.iter_mut() {
            callback(&event);
        }
    }
}

pub trait NotifierExt {
    fn shared() -> &'static mut Arc<Notifier<Option<Win::MSG>>>;
}

impl NotifierExt for Notifier<Option<Win::MSG>> {
    fn shared() -> &'static mut Arc<Notifier<Option<Win::MSG>>> {
        static mut CONF: MaybeUninit<Arc<Notifier<Option<Win::MSG>>>> = MaybeUninit::uninit();
        static ONCE: Once = Once::new();
    
        ONCE.call_once(|| unsafe {
            CONF.as_mut_ptr().write(Arc::new(Notifier::new()));
        });

        unsafe { &mut *CONF.as_mut_ptr() }
    }
}
