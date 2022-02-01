#![allow(non_snake_case)]
#![warn(unused_variables)]

use windows::{
    core::*,
};

mod Win;
mod muffui;

use std::rc::Rc;
use std::cell::RefCell;
use crate::muffui::*;
use SharedProps::*;


pub struct MyView {
    title: Rc<RefCell<String>>,
    index: Rc<RefCell<isize>>,
    text: Rc<RefCell<String>>,
    debugInfo: Rc<RefCell<String>>,
    selected: Rc<RefCell<usize>>,
    selectedText: Rc<RefCell<String>>,
    selectedIndex: Rc<RefCell<usize>>,
    checkedItem1: Rc<RefCell<bool>>,
}

impl MyView {
    pub fn new() -> Self {
        Self {
            index: Default::default(),
            title: Rc::new(RefCell::new(String::from("test"))),
            text: Rc::new(RefCell::new(String::from("edit text"))),
            debugInfo: Rc::new(RefCell::new(String::from("..."))),
            selected: Rc::new(RefCell::new(0)),
            selectedText: Rc::new(RefCell::new(String::from(""))),
            selectedIndex: Rc::new(RefCell::new(0)),
            checkedItem1: Rc::new(RefCell::new(false)),
        }
    }
}

impl Renderable for MyView {
    fn childs(&self) -> Option<Rc<dyn Renderable>> {
        let title: &str = &self.title.borrow();
        let text: &str = &self.text.borrow();
        let debugInfo: &str = &self.debugInfo.borrow();
        let selected: &usize = &self.selected.borrow();
        let selectedText: &str = &self.selectedText.borrow();
        let selectedIndex: &usize = &self.selectedIndex.borrow();
        let checkedItem1: &bool = &self.checkedItem1.borrow();

        Some(Rc::from(Window::new([
            SP::ClassName("window#1"), ControlId(0), Anchor(ANF_NONE), SP::Title("window title"), SP::FontFace("Tahoma"),
            Width(410), Height(410)
        ]).didResize({
            let debugInfo = Rc::clone(&self.debugInfo);
            move|props|{
                let mut debugInfo = debugInfo.borrow_mut();
                *debugInfo = String::from(format!("{:?}", props));
        }}).content(||(
            Panel::new([SP::Title("testing title"), ControlId(103), Anchor(ANF_TOP|ANF_LEFT|ANF_RIGHT)]).posX(4).posY(4).width(385).height(35).content(||(
                CheckBox::new([SP::Title(text), ControlId(102), Anchor(ANF_TOP|ANF_LEFT), SP::FontFace(text)]).posX(5).posY(5).width(120).height(25).content(Command::new({
                    move|event: Vec<SharedProps>| {

                    }
                }))
                ,
                TextBox::new([ControlId(201)]).posX(140).width(240)
            ))
            ,
            Panel::new([SP::Title("testing title"), ControlId(104), Anchor(ANF_TOP|ANF_LEFT|ANF_BOTTOM|ANF_RIGHT)]).posX(4).posY(40).width(385).height(300).content(||(
                Label::new([SP::Title("testing label"), ControlId(105), Anchor(ANF_TOP|ANF_LEFT), SP::FontFace("Monaco")]).posX(5).posY(5).width(125).height(25)
                ,
                Button::new([SP::Title(title), ControlId(106), Anchor(ANF_TOP|ANF_LEFT)]).posX(120).posY(5).width(65).height(25).content({
                    let index = Rc::clone(&self.index);
                    let title = Rc::clone(&self.title);
                    let debugInfo = Rc::clone(&self.debugInfo);
                    move |_|{
                        let mut index = index.borrow_mut();
                        let mut title = title.borrow_mut();
                        let mut debugInfo = debugInfo.borrow_mut();
                        *index += 1;
                        *title = String::from(format!("OK: {}", index));
                        *debugInfo = String::from(format!("Click: {}!!!!", title));
                }})
                ,
                GroupBox::new([SP::Title("group box"), ControlId(107), Anchor(ANF_TOP|ANF_LEFT|ANF_BOTTOM|ANF_RIGHT)]).posX(5).posY(40).width(300).height(200).content(||(
                    Label::new([SP::Title("Text box:"), ControlId(108), Anchor(ANF_TOP|ANF_LEFT)]).posX(6).posY(20).width(75).height(25),
                    TextBox::new([SP::Title(text), ControlId(109), Anchor(ANF_TOP|ANF_LEFTRIGHT)]).posX(85).posY(20).width(120).height(25).content({
                        let text = Rc::clone(&self.text);
                        move|event: Vec<SharedProps>|{
                        let mut text = text.borrow_mut();
                        if let Some(Title(newTitle)) = event.prop(&SP::Title("any")) {
                            *text = newTitle.to_string();
                        }
                    }})
                    ,
                    RadioBox::new([SP::Title("radio box 1"), ControlId(110), Anchor(ANF_TOP|ANF_LEFT), Selected(*selected == 1)]).posX(6).posY(45).width(120).height(25).content({
                        let selected = Rc::clone(&self.selected);
                        move|event|{
                            let mut selected = selected.borrow_mut();
                            *selected = 1;
                        }
                    })
                    ,
                    RadioBox::new([SP::Title("radio box 2"), ControlId(111), Anchor(ANF_TOP|ANF_LEFT), Selected(*selected == 2)]).posX(6).posY(70).width(120).height(25).content({
                        let selected = Rc::clone(&self.selected);
                        move|event|{
                            let mut selected = selected.borrow_mut();
                            *selected = 2;
                        }
                    })
                    ,
                    CheckBox::new([SP::Title("Check box 1"), ControlId(112), Anchor(ANF_TOP|ANF_LEFT), Selected(*checkedItem1)]).posX(6).posY(95).width(120).height(25).content({
                        let checkedItem1 = Rc::clone(&self.checkedItem1);
                        move|event|{
                            let mut checked = checkedItem1.borrow_mut();
                            *checked = !*checked;
                        }
                    })
                    ,
                    CheckBox::new([SP::Title("Check box 2"), ControlId(113), Anchor(ANF_TOP|ANF_LEFT), Selected(false)]).posX(6).posY(120).width(120).height(25).content({

                        move|event|{

                        }
                    })
                    ,
                    Select::new([SP::Title(selectedText), ControlId(200), Anchor(ANF_TOP|ANF_LEFT), SelectItems(vec![
                        String::from("first item"), String::from("Second item"), String::from("Third item")
                    ]), SelectedIndex(*selectedIndex)]).posX(6).posY(170).width(160).height(120).didChange({
                        let selectedText = Rc::clone(&self.selectedText);
                        let selectedIndex = Rc::clone(&self.selectedIndex);
                        move|event: Vec<SharedProps>|{
                            let mut selectedText = selectedText.borrow_mut();
                            if let Some(Title(newTitle)) = event.prop(&SP::Title("any")) {
                                *selectedText = newTitle.to_string();
                            }
                            let mut selectedIndex = selectedIndex.borrow_mut();
                            if let Some(SelectedIndex(newIndex)) = event.prop(&SelectedIndex(0)) {
                                *selectedIndex = *newIndex;
                            }
                    }})
                ))
                ,
            ))
            ,
            Label::new([SP::Title(debugInfo), Anchor(ANF_BOTTOM|ANF_LEFTRIGHT), ControlId(115), SP::FontFace(text)]).posX(4).posY(350).width(1024).height(25)
        ))))
    }
}

impl MainApp for App {
    type View = MyView;
    fn view(&self) -> Option<Self::View> {
        Some(MyView::new())
    }
}

fn main() -> Result<()> {

    let app = App::shared();

    app.lock()
        .and_then(|a|Ok(a)).as_mut().map(|a|a.run()).map(|_|())
        .or_else(|_|Err(Error::new(Win::HRESULT(1), "locking app mutex failed".into())))
}
