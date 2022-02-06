#![allow(non_snake_case)]

use std::sync::Arc;
use std::rc::Rc;
use std::sync::Mutex;
use crate::muffui::*;
use SharedProps::*;
use crate::Win;

pub struct Window {
    pub children: Option<Rc<dyn Renderable>>,
    pub props: Vec<SharedProps>,
}

impl Default for Window {
    fn default() -> Self {
        Self {
            children: Default::default(),
            props: vec![],
        }
    }
}

impl Window {
    pub fn new<T: Into<Vec<SharedProps>>>(props: T) -> Self {
        let defaultProps = vec![
            SP::Renderer("window"),
            SP::ClassName("window"),
            Width(1024),
            Height(768),
        ];
        Self {
            props: defaultProps.merge(props.into()),
            ..Default::default()
        }
    }
    #[allow(dead_code)]
    pub fn className(self, className: &str) -> Self {
        Self {
            props: self.props.merge(vec![SP::ClassName(className)]),
            ..self
        }
    }
    #[allow(dead_code)]
    pub fn title(self, title: &str) -> Self {
        Self {
            props: self.props.merge(vec![SP::Title(title)]),
            ..self
        }
    }
    #[allow(dead_code)]
    pub fn width(self, width: i32) -> Self {
        Self {
            props: self.props.merge(SharedProps::Width(width)),
            ..self
        }
    }
    #[allow(dead_code)]
    pub fn height(self, height: i32) -> Self {
        Self {
            props: self.props.merge(vec![SharedProps::Height(height)]),
            ..self
        }
    }
    #[allow(dead_code)]
    pub fn didResize<C: Into<Command<Vec<SharedProps>>>>(self, handler: C) -> Self {
        Self {
            props: self.props.merge(SharedProps::DidResize(Arc::new(Mutex::new(handler.into())))),
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

impl Renderable for Window {
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
        self.props.clone().merge([SP::DidResize({
            let didResize = self.props.prop(&SP::DidResize(|_|{})).map(|d|d.clone());
            move|props: Vec<SharedProps>|{
                let didResize = didResize.clone();
                if let Some(SharedProps::DidResize(didResize)) = didResize {
                    didResize.lock().unwrap().exec(props.clone());
                }
            }
        })])
    }
}
