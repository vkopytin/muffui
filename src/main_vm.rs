use std::sync::Arc;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::Once;
use std::mem::MaybeUninit;
use crate::muffui::utils;

pub struct MainViewModel {
    pub newTitle: Rc<RefCell<String>>,
    pub items: Rc<RefCell<Vec<(usize, String, bool)>>>,
    pub showAll: Rc<RefCell<usize>>,
}

impl MainViewModel {
    pub fn new() -> Self {
        Self {
            newTitle: Rc::from(RefCell::new(String::from(""))),
            items: Rc::from(RefCell::new(vec![])),
            showAll: Rc::new(RefCell::new(0)),
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

    pub fn setShowAll(&self, val: usize) {
        let mut showAll = self.showAll.borrow_mut();
        match val {
            1 => *showAll = 1,
            2 => *showAll = 2,
            _ => *showAll = 0,
        }
    }

    pub fn getItems(&self) -> Vec<(usize, String, bool)> {
        let mut items = vec![];
        let showAll = self.showAll.borrow();
        for (id, name, done) in self.items.borrow().iter() {
            match *showAll {
                1 if !*done => items.push((*id, name.clone(), *done)),
                2 if *done => items.push((*id, name.clone(), *done)),
                0 => items.push((*id, name.clone(), *done)),
                _ => (),
            }
        }

        items
    }

    pub fn getCompleted(&self) -> usize {
        self.items.borrow().iter().filter(|(_,_,done)|*done).fold(0, |r, _|r + 1)
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

    pub fn removeToDo(&self, itemId: usize) {
        let index = self.items.borrow().iter().position(|(id, _, _)|*id == itemId);
        if let Some(index) = index {
            let mut items = self.items.borrow_mut();
            items.remove(index);
        }
    }

    pub fn clearCompleted(&self) {
        let completed = self.items.borrow().iter()
            .filter(|(_, _, done)|*done)
            .map(|(id, _, _)|*id)
            .collect::<Vec<_>>();

        for index in completed {
            self.removeToDo(index);
        }
    }

    pub fn completeAll(&self) {
        let incomplete = self.items.borrow().iter().filter(|(_, _, done)|!*done).map(|i|i.clone()).collect::<Vec<_>>();
        for (id, name, _) in incomplete {
            self.updateToDo((id, name, true));
        }
    }
}
