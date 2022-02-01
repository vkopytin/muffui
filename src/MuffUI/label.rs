#![allow(non_snake_case)]

use std::rc::Rc;
use crate::Win;
use crate::muffui::*;
use crate::muffui::SharedProps::*;


#[derive(Clone)]
pub struct Label {
    pub children: Option<Rc<dyn Renderable>>,
    pub props: Vec<SharedProps>,
}

impl Default for Label {
    fn default() -> Self {
        Self {
            children: Default::default(),
            props: vec![],
        }
    }
}

impl Renderable for Label {
    fn childs(&self) -> Option<Rc<dyn Renderable>> {
        Some(self.children.as_ref()?.clone())
    }

    fn render(&self, context: Box<UIContext>, parent: &str, index: &str, msg: Option<Win::MSG>) -> Box<UIContext> {
        let context = context.render(parent, index, self, msg);
        match &self.children {
            Some(children) => children.render(context, index, &format!("{}:1", index), msg),
            _ => context,
        }
    }

    fn toViewState(&self) -> Vec<SharedProps> {
        self.props.iter().map(|item|item.clone()).collect()
    }
}

impl Label {
    pub fn new<T: Into<Vec<SharedProps>>>(props: T) -> Self {
        let defaultProps = vec![
            SP::ClassName("Static"),
            SP::Renderer("label"),
        ];
        Self {
            props: defaultProps.merge(props.into()),
            ..Default::default()
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
    #[allow(dead_code)]
    pub fn content<U: 'static + Renderable>(self, mut children: impl FnMut() -> U) -> Self {
        Self {
            children: Some(Rc::new(children())),
            ..self
        }
    }
}
