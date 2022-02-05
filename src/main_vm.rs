use std::sync::Arc;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::Once;
use std::mem::MaybeUninit;
use crate::muffui::utils;

pub struct MainViewModel {
    pub newTitle: Rc<RefCell<String>>,
    pub items: Rc<RefCell<Vec<(usize, String, bool)>>>,
}

impl MainViewModel {
    pub fn new() -> Self {
        Self {
            newTitle: Rc::from(RefCell::new(String::from(""))),
            items: Rc::from(RefCell::new(vec![])),
        }
    }

    pub fn shared() -> Arc<MainViewModel> {
        static mut CONF: MaybeUninit<Arc<MainViewModel>> = MaybeUninit::uninit();
        static ONCE: Once = Once::new();
    
        ONCE.call_once(|| unsafe {
            CONF.as_mut_ptr().write(Arc::new(MainViewModel::new()));
        });

        unsafe { (*CONF.as_ptr()).clone() }
    }

    pub fn createToDo(&self) {
        let mut items = self.items.borrow_mut();
        let id = utils::uniqId();
        items.push((id, String::from(&*self.newTitle.borrow()), false));
        *self.newTitle.borrow_mut() = String::from("");
    }

    pub fn updateToDo(&self, props: (usize, String, bool)) {
        let (id, name, isFinished) = props;
        if let Some(mut item) = self.items.borrow_mut().iter_mut().find(|i|i.0 == id) {
            item.1 = name;
            item.2 = isFinished;
        }
    }

    pub fn removeToDo(&self, id: usize) {
        let index = self.items.borrow().iter().position(|i|i.0 == id);
        if let Some(index) = index {
            let mut items = self.items.borrow_mut();
            items.remove(index);
        }
    }
}
