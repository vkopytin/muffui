#![allow(non_snake_case)]

use std::sync::Once;
use std::mem::MaybeUninit;
use std::collections::HashMap;
use std::sync::Mutex;
use crate::Win;
use crate::muffui::*;

pub trait MainApp {
    type View: Renderable;
    fn view(&self) -> Option<Self::View>;
}

pub struct App {
    
}

impl App where Self: MainApp {
    pub fn new() -> Self {
        Self {
            
        }
    }

    pub fn shared() -> &'static Mutex<App> {
        static mut CONF: MaybeUninit<Mutex<App>> = MaybeUninit::uninit();
        static ONCE: Once = Once::new();
    
        ONCE.call_once(|| unsafe {
            CONF.as_mut_ptr().write(Mutex::new(App::new()));
        });

        unsafe { &*CONF.as_ptr() }
    }

    pub fn run(&mut self) -> windows::core::Result<()> {
        Win::CoInitializeEx()?;

        let mut context = UIContext::create();
        if let Some(view) = self.view() {
            context = view.render(context, "/", "0", None);
            Notifier::shared().try_lock().ok().as_mut().map(|e|e.register(move|msg|{
                EventHub::shared().try_lock().ok().as_mut().map(|e|{
                    if let Some(msg) = msg {
                        let msg = msg.clone();
                        e.enqueueEvent(msg);
                        let mut prev = context.clone();
                        prev.prevItems = HashMap::new();
                        context = view.render(prev, "/", "0", Some(msg));
                        context = context.clone().clean();
                        for (_, ci) in context.items.iter() {
                            e.putListener(ci.hwnd, ci.listeners.clone());
                        }
                    }
                    if e.dispatchEvents() {
                        context = view.render(context.clone(), "/", "0", None);
                    }
                });
            }));

            let mut msg = Win::MSG { ..Default::default() };
            while Win::WM_QUIT != msg.message {
                if let Some(msg2) = Win::PeekMessage() {
                    msg = msg2;

                    Win::TranslateMessage(&mut msg);
                    Win::DispatchMessage(&mut msg);

                    Notifier::shared().try_lock().ok().as_mut().map(|e|e.notify(Some(msg)));
                } else {
                    std::thread::sleep(std::time::Duration::from_millis(100));
                    Notifier::shared().try_lock().ok().as_mut().map(|e|e.notify(None));
                }
            }
        }

        Ok(())
    }
}

