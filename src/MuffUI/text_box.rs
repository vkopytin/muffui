#![allow(non_snake_case)]

use std::sync::Arc;
use std::rc::Rc;
use std::sync::Mutex;
use crate::Win;
use crate::muffui::*;
use crate::muffui::SharedProps::*;

#[derive(Clone)]
pub struct TextBox {
    props: Vec<SharedProps>,
}

impl Default for TextBox {
    fn default() -> Self {
        Self {
            props: vec![]
        }
    }
}

impl Renderable for TextBox {
    fn childs(&self) -> Option<Rc<dyn Renderable>> {
        None
    }

    fn render(&self, context: Box<UIContext>, parent: &str, index: &str, msg: Option<Win::MSG>) -> Box<UIContext> {
        context.render(parent, index, self, msg)
    }

    fn toViewState(&self) -> Vec<SharedProps> {
        self.props.iter().map(|item|item.clone()).collect()
    }
}

impl TextBox {
    pub fn new<T: Into<Vec<SharedProps>>>(props: T) -> Self {
        let defaultProps = vec![
            SP::ClassName("Edit"),
            SP::Renderer("text-box"),
        ];
        Self {
            props: defaultProps.merge(props.into()),
        }
    }
    #[allow(dead_code)]
    pub fn title(self, title: &str) -> Self {
        Self {
            props: self.props.merge(SP::Title(title)),
            ..self
        }
    }

    pub fn posX(self, posX: i32) -> Self {
        Self {
            props: self.props.merge(PosX(posX)),
            ..self
        }
    }

    pub fn posY(self, posY: i32) -> Self {
        Self {
            props: self.props.merge(PosY(posY)),
            ..self
        }
    }

    pub fn width(self, width: i32) -> Self {
        Self {
            props: self.props.merge(Width(width)),
            ..self
        }
    }

    pub fn height(self, height: i32) -> Self {
        Self {
            props: self.props.merge(Height(height)),
            ..self
        }
    }

    pub fn content<C: Into<Command<Vec<SharedProps>>>>(self, handler: C) -> Self {
        Self {
            props: self.props.merge(SharedProps::DidChange(Arc::new(Mutex::new(handler.into())))),
            ..self
        }
    }
}
