use std::sync::Arc;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::Once;
use std::mem::MaybeUninit;

pub struct MainViewModel {
    pub newTitle: Rc<RefCell<String>>,
    pub items: Rc<RefCell<Vec<String>>>,
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
        items.push(String::from(&*self.newTitle.borrow()));
        *self.newTitle.borrow_mut() = String::from("");
    }
}
