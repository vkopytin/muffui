#![allow(non_snake_case)]
use std::fmt;
use std::rc::Rc;
use std::sync::Mutex;
use std::sync::Arc;
use crate::muffui::Command;

#[derive(Clone)]
pub enum SharedProps {
    Renderer(Rc<str>),
    ControlId(i32),
    ClassName(Rc<str>),
    Title(Rc<str>),
    Width(i32),
    Height(i32),
    PosX(i32),
    PosY(i32),
    FontFace(Rc<str>),
    Anchor(usize),
    Selected(bool),
    SelectItems(Vec<String>),
    SelectedIndex(usize),

    DidClick(Arc<Mutex<Command<Vec<SharedProps>>>>),
    DidChange(Arc<Mutex<Command<Vec<SharedProps>>>>),
    DidResize(Arc<Mutex<Command<Vec<SharedProps>>>>),
}

impl fmt::Debug for SharedProps {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SharedProps::Renderer(t) => write!(f, "Renderer({})", t),
            SharedProps::ControlId(t) => write!(f, "ControlId({})", t),
            SharedProps::ClassName(t) => write!(f, "ClassName({})", t),
            SharedProps::Title(t) => write!(f, "Title({})", t),
            SharedProps::Width(t) => write!(f, "Width({})", t),
            SharedProps::Height(t) => write!(f, "Height({})", t),
            SharedProps::PosX(t) => write!(f, "PosX({})", t),
            SharedProps::PosY(t) => write!(f, "PosY({})", t),
            SharedProps::FontFace(t) => write!(f, "FontFace({})", t),
            SharedProps::Anchor(t) => write!(f, "Anchor({})", t),
            SharedProps::Selected(t) => write!(f, "Selected({})", t),
            SharedProps::SelectItems(t) => write!(f, "SelectItems({:?})", t),
            SharedProps::SelectedIndex(t) => write!(f, "SelectedIndex({})", t),

            SharedProps::DidChange(_) => write!(f, "fn:didChange"),
            SharedProps::DidClick(_) => write!(f, "fn:didClick"),
            SharedProps::DidResize(_) => write!(f, "fn:didResize"),
        }
    }
}

impl fmt::Display for SharedProps {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

pub struct SP;

impl SP {
    pub fn ClassName(className: &str) -> SharedProps {
        SharedProps::ClassName(Rc::from(className))
    }
    pub fn Renderer(name: &str) -> SharedProps {
        SharedProps::Renderer(Rc::from(name))
    }
    pub fn Title(title: &str) -> SharedProps {
        SharedProps::Title(Rc::from(title))
    }
    pub fn FontFace(face: &str) -> SharedProps {
        SharedProps::FontFace(Rc::from(face))
    }
    #[allow(dead_code)]
    pub fn DidChange<C: Into<Command<Vec<SharedProps>>>>(handler: C) -> SharedProps {
        SharedProps::DidChange(Arc::new(Mutex::new(handler.into())))
    }
}

impl From<SharedProps> for Vec<SharedProps> {
    fn from(value: SharedProps) -> Self {
        vec![value]
    }
}

pub trait VectorExtention {
    fn prop(&self, variant: &SharedProps) -> Option<&SharedProps>;
    fn merge<T: Into<Vec<SharedProps>>>(self, right: T) -> Self;
    fn update<T: Into<Vec<SharedProps>>>(self, right: T) -> Self;
}

impl VectorExtention for Vec<SharedProps> {
    fn prop(&self, variant: &SharedProps) -> Option<&SharedProps> {
        self.iter().find_map(|d|{
            if std::mem::discriminant(d) == std::mem::discriminant(variant) {
                Some(d)
            } else {
                None
            }
        })
    }
    fn merge<T: Into<Vec<SharedProps>>>(self, right: T) -> Self {
        let mut difference = vec![];
        let right = right.into();
        for item in self.into_iter() {
            if right.prop(&item).is_none() {
                difference.push(item);
            }
        }
        difference.into_iter().chain(right).collect()
    }
    fn update<T: Into<Vec<SharedProps>>>(self, right: T) -> Self {
        let right = right.into();
        let mut inst = self;

        for i in right.into_iter() {
            let prop = inst.prop(&i);
            if let Some(_) = prop {
                inst = inst.merge(i);
            }
        }

        inst
    }
}
