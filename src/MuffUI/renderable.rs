#![allow(non_snake_case)]

use std::rc::Rc;
use crate::Win;
use crate::muffui::uicontext::UIContext;
use crate::muffui::SharedProps;


pub trait Renderable {
    fn childs(&self) -> Option<Rc<dyn Renderable>>;
    fn render(&self, context: Box<UIContext>, parent: &str, index: &str, msg: Option<Win::MSG>) -> Box<UIContext> {
        let childs = self.childs();
        match childs {
            Some(children) => children.render(context, parent, index, msg),
            _ => context,
        }
    }

    fn toViewState(&self) -> Vec<SharedProps> {
        vec![]
    }
}

pub struct EmptyRenderable {

}

impl Renderable for EmptyRenderable {
    fn childs(&self) -> Option<Rc<dyn Renderable>> {
        None
    }
    fn render(&self, context: Box<UIContext>, _: &str, _: &str, _: Option<Win::MSG>) -> Box<UIContext> {
        context
    }
    fn toViewState(&self) -> Vec<SharedProps> {
        vec![]
    }
}

pub enum ContentArgs<A: Renderable, B: Renderable, C: Renderable, D: Renderable, E: Renderable, F: Renderable, G: Renderable> {
    OneArg(A),
    TwoArgs(A, B),
    ThreeArgs(A, B, C),
    FourArgs(A, B, C, D),
    FiveArgs(A, B, C, D, E),
    SixArgs(A, B, C, D, E, F),
    SevenArgs(A, B, C, D, E, F, G),
}

impl<A, B, C, D, E, F, G> ContentArgs<A, B, C, D, E, F, G> where A: Renderable, B: Renderable, C: Renderable, D: Renderable, E: Renderable, F: Renderable, G: Renderable {
    #[allow(dead_code)]
    pub fn new<T>(args: T) -> ContentArgs<A, B, C, D, E, F, G>
        where T: Into<ContentArgs<A, B, C, D, E, F, G>>
    {
        args.into()
    }
}

impl<A: Renderable> From<A> for ContentArgs<A, EmptyRenderable, EmptyRenderable, EmptyRenderable, EmptyRenderable, EmptyRenderable, EmptyRenderable> {
    fn from(a: A) -> ContentArgs<A, EmptyRenderable, EmptyRenderable, EmptyRenderable, EmptyRenderable, EmptyRenderable, EmptyRenderable> {
        ContentArgs::OneArg(a)
    }
}

impl<A: Renderable, B: Renderable> From<(A, B)> for ContentArgs<A, B, EmptyRenderable, EmptyRenderable, EmptyRenderable, EmptyRenderable, EmptyRenderable> {
    fn from((a, b): (A, B)) -> ContentArgs<A, B, EmptyRenderable, EmptyRenderable, EmptyRenderable, EmptyRenderable, EmptyRenderable> {
        ContentArgs::TwoArgs(a, b)
    }
}

impl<A: Renderable, B: Renderable, C: Renderable> From<(A, B, C)> for ContentArgs<A, B, C, EmptyRenderable, EmptyRenderable, EmptyRenderable, EmptyRenderable> {
    fn from((a, b, c): (A, B, C)) -> ContentArgs<A, B, C, EmptyRenderable, EmptyRenderable, EmptyRenderable, EmptyRenderable> {
        ContentArgs::ThreeArgs(a, b, c)
    }
}

impl<A: Renderable, B: Renderable, C: Renderable, D: Renderable> From<(A, B, C, D)> for ContentArgs<A, B, C, D, EmptyRenderable, EmptyRenderable, EmptyRenderable> {
    fn from((a, b, c, d): (A, B, C, D)) -> ContentArgs<A, B, C, D, EmptyRenderable, EmptyRenderable, EmptyRenderable> {
        ContentArgs::FourArgs(a, b, c, d)
    }
}

impl<A: Renderable, B: Renderable, C: Renderable, D: Renderable, E: Renderable> From<(A, B, C, D, E)> for ContentArgs<A, B, C, D, E, EmptyRenderable, EmptyRenderable> {
    fn from((a, b, c, d, e): (A, B, C, D, E)) -> ContentArgs<A, B, C, D, E, EmptyRenderable, EmptyRenderable> {
        ContentArgs::FiveArgs(a, b, c, d, e)
    }
}

impl<A: Renderable, B: Renderable, C: Renderable, D: Renderable, E: Renderable, F: Renderable> From<(A, B, C, D, E, F)> for ContentArgs<A, B, C, D, E, F, EmptyRenderable> {
    fn from((a, b, c, d, e, f): (A, B, C, D, E, F)) -> ContentArgs<A, B, C, D, E, F, EmptyRenderable> {
        ContentArgs::SixArgs(a, b, c, d, e, f)
    }
}

impl<A: Renderable, B: Renderable, C: Renderable, D: Renderable, E: Renderable, F: Renderable, G: Renderable> From<(A, B, C, D, E, F, G)> for ContentArgs<A, B, C, D, E, F, G> {
    fn from((a, b, c, d, e, f, g): (A, B, C, D, E, F, G)) -> ContentArgs<A, B, C, D, E, F, G> {
        ContentArgs::SevenArgs(a, b, c, d, e, f, g)
    }
}

impl<A: Renderable, B: Renderable, C: Renderable, D: Renderable, E: Renderable, F: Renderable, G: Renderable> Renderable for ContentArgs<A, B, C, D, E, F, G> {
    fn childs(&self) -> Option<Rc<dyn Renderable>> {
        None
    }
    fn render(&self, context: Box<UIContext>, parent: &str, index: &str, msg: Option<Win::MSG>) -> Box<UIContext> {
        match self {
            ContentArgs::SevenArgs(a, b, c, d, e, f, g) => {
                let context = a.render(context, parent, &format!("{}_1", index), msg);
                let context = b.render(context, parent, &format!("{}_2", index), msg);
                let context = c.render(context, parent, &format!("{}_3", index), msg);
                let context = d.render(context, parent, &format!("{}_4", index), msg);
                let context = e.render(context, parent, &format!("{}_5", index), msg);
                let context = f.render(context, parent, &format!("{}_6", index), msg);
                g.render(context, parent, &format!("{}_7", index), msg)
            },
            ContentArgs::SixArgs(a, b, c, d, e, f) => {
                let context = a.render(context, parent, &format!("{}_1", index), msg);
                let context = b.render(context, parent, &format!("{}_2", index), msg);
                let context = c.render(context, parent, &format!("{}_3", index), msg);
                let context = d.render(context, parent, &format!("{}_4", index), msg);
                let context = e.render(context, parent, &format!("{}_5", index), msg);
                f.render(context, parent, &format!("{}_6", index), msg)
            },
            ContentArgs::FiveArgs(a, b, c, d, e) => {
                let context = a.render(context, parent, &format!("{}_1", index), msg);
                let context = b.render(context, parent, &format!("{}_2", index), msg);
                let context = c.render(context, parent, &format!("{}_3", index), msg);
                let context = d.render(context, parent, &format!("{}_4", index), msg);
                e.render(context, parent, &format!("{}_5", index), msg)
            },
            ContentArgs::FourArgs(a, b, c, d) => {
                let context = a.render(context, parent, &format!("{}_1", index), msg);
                let context = b.render(context, parent, &format!("{}_2", index), msg);
                let context = c.render(context, parent, &format!("{}_3", index), msg);
                d.render(context, parent, &format!("{}_4", index), msg)
            },
            ContentArgs::ThreeArgs(a, b, c) => {
                let context = a.render(context, parent, &format!("{}_1", index), msg);
                let context = b.render(context, parent, &format!("{}_2", index), msg);
                c.render(context, parent, &format!("{}:3", index), msg)
            },
            ContentArgs::TwoArgs(a, b) => {
                let context = a.render(context, parent, &format!("{}_1", index), msg);
                b.render(context, parent, &format!("{}_2", index), msg)
            },
            ContentArgs::OneArg(a) => a.render(context, parent, &format!("{}_1", index), msg),
        }
    }
    fn toViewState(&self) -> Vec<SharedProps> {
        vec![]
    }
}

pub struct ForEach<T: Renderable> {
    pub children: Vec<T>,
}

impl<T: Renderable> Renderable for ForEach<T> {
    fn childs(&self) -> Option<Rc<dyn Renderable>> {
        None
    }

    fn render(&self, context: Box<UIContext>, parent: &str, index: &str, msg: Option<Win::MSG>) -> Box<UIContext> {
        let mut idx = 0;
        self.children.iter().fold(context, |res, item|{
            let res = item.render(res, parent, &format!("{}[{}]", index, idx), msg);
            idx += 1;
            res
        })
    }

    fn toViewState(&self) -> Vec<SharedProps> {
        vec![]
    }
}

impl<T: Renderable> ForEach<T> {
    pub fn new<A>(items: Vec<A>, f: impl Fn(A, i32) -> T) -> Self {
        let mut idx = 0;
        let children = items.into_iter().map(|i|{
            let res = f(i, idx);
            idx += 1;
            res
        }).collect();
        Self {
            children,
        }
    }
}
