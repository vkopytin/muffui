#![allow(non_snake_case)]

use std::rc::Rc;
use std::sync::Mutex;
use std::sync::Arc;
use crate::Win;
use crate::muffui::*;
use crate::muffui::SharedProps::*;


#[derive(Clone)]
pub struct Select {
    pub children: Option<Rc<dyn Renderable>>,
    pub props: Vec<SharedProps>,
}

impl Default for Select {
    fn default() -> Self {
        Self {
            children: Default::default(),
            props: vec![],
        }
    }
}

impl Renderable for Select {
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

impl Select {
    #[allow(dead_code)]
    pub fn new<T: Into<Vec<SharedProps>>>(props: T) -> Self {
        let defaultProps = vec![
            SP::ClassName("ComboBox"),
            SP::Renderer("select"),
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
    #[allow(dead_code)]
    pub fn posX(self, posX: i32) -> Self {
        Self {
            props: self.props.merge(PosX(posX)),
            ..self
        }
    }
    #[allow(dead_code)]
    pub fn posY(self, posY: i32) -> Self {
        Self {
            props: self.props.merge(PosY(posY)),
            ..self
        }
    }
    #[allow(dead_code)]
    pub fn width(self, width: i32) -> Self {
        Self {
            props: self.props.merge(Width(width)),
            ..self
        }
    }
    #[allow(dead_code)]
    pub fn height(self, height: i32) -> Self {
        Self {
            props: self.props.merge(Height(height)),
            ..self
        }
    }
    #[allow(dead_code)]
    pub fn didChange<C: Into<Command<Vec<SharedProps>>>>(self, handler: C) -> Self {
        Self {
            props: self.props.merge(SharedProps::DidChange(Arc::new(Mutex::new(handler.into())))),
            ..self
        }
    }
    #[allow(dead_code)]
    pub fn content<A: 'static, B: 'static, C: 'static, D: 'static, E: 'static, F: 'static, G: 'static, T: 'static, FF>(self, mut children: FF) -> Self
        where
            A: Renderable,
            B: Renderable,
            C: Renderable,
            D: Renderable,
            E: Renderable,
            F: Renderable,
            G: Renderable,
            T: Into<ContentArgs<A, B, C, D, E, F, G>>,
            FF: FnMut() -> T
    {
        let args: ContentArgs<A, B, C, D, E, F, G> = children().into();
        Self {
            children: Some(Rc::new(args)),
            ..self
        }
    }
}
