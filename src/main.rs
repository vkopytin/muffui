#![allow(non_snake_case)]
#![warn(unused_variables)]

use windows::{
    core::*,
};

mod main_vm;
mod muffui;

use crate::muffui::win as Win;
use std::rc::Rc;
use std::sync::Arc;
use std::cell::RefCell;
use crate::muffui::*;
use SharedProps::*;
use crate::main_vm::MainViewModel;


pub struct MyView {
    vm: Arc<MainViewModel>,
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
            vm: MainViewModel::shared(),
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
        let newTitle: &str = &self.vm.newTitle.borrow();

        Some(Rc::from(Window::new([
            SP::ClassName("window#1"), ControlId(0), Anchor(ANF_NONE), SP::Title("TODO: Example"), SP::FontFace("Tahoma"),
            Width(500), Height(610)
        ]).didResize({
            let debugInfo = Rc::clone(&self.debugInfo);
            move|props|{
                let mut debugInfo = debugInfo.borrow_mut();
                *debugInfo = String::from(format!("{:?}", props));
        }}).content(||(
            Panel::new([SP::Title("create new todo"), ControlId(103), Anchor(ANF_TOP|ANF_LEFTRIGHT)]).posX(4).posY(4).width(475).height(35).content(||(
                CheckBox::new([SP::Title("Mark All"), ControlId(102), Anchor(ANF_TOP|ANF_LEFT)]).posX(5).posY(5).width(70).height(25).content(Command::new({
                    move|event: Vec<SharedProps>| {

                    }
                }))
                ,
                Label::new([SP::Title("New todo title:")]).posX(76).posY(9).width(120).height(25)
                ,
                TextBox::new([ControlId(201), Anchor(ANF_TOP| ANF_LEFTRIGHT)]).title(newTitle).posX(160).posY(6).width(240).height(21).content({
                    let newTitle = Rc::clone(&self.vm.newTitle);
                    move|event: Vec<SharedProps>|{
                        let mut newTitle = newTitle.borrow_mut();
                        if let Some(Title(v)) = event.prop(&SP::Title("")) {
                            *newTitle = v.to_string();
                        }
                    }
                })
                ,
                Button::new([SP::Title("Save"), ControlId(202), Anchor(ANF_TOP|ANF_RIGHT)]).posX(406).posY(4).width(40).height(24).content({
                    let vm = self.vm.clone();
                    move|_|{
                        vm.createToDo();
                    }
                })
            ))
            ,
            Panel::new([ControlId(203), Anchor(ANF_TOPBOTTOM|ANF_LEFTRIGHT)]).posX(4).posY(40).width(475).height(200).content(||
                ForEach::new(self.vm.items.borrow().iter().map(|i|i.clone()).collect::<Vec<_>>(), |(id, name, isFinished), index|(
                    Panel::new([ControlId(300 + index), Anchor(ANF_LEFTRIGHT|ANF_TOP)]).posX(4).posY(2 + index * 29).height(27).width(400).content(||(
                        CheckBox::new([Selected(isFinished), SP::Title("Complete"), ControlId(400 + index), Anchor(ANF_TOP|ANF_LEFT)]).posX(4).width(80).posY(1).height(24).content({
                            let vm = self.vm.clone();
                            let name = name.clone();
                            move|args: Vec<SharedProps>|{
                                vm.updateToDo((id, name.clone(), !isFinished));
                        }})
                        ,
                        TextBox::new([SP::Title(name.as_str()), ControlId(400 + 2 * index), Anchor(ANF_TOP|ANF_LEFTRIGHT)]).posX(85).posY(1).width(200).height(24).content({
                            let vm = self.vm.clone();
                            move|args: Vec<SharedProps>|{
                                if let Some(Title(title)) = args.prop(&SP::Title("")) {
                                    vm.updateToDo((id, title.to_string(), isFinished));
                                }
                            }
                        })
                        ,
                        Button::new([SP::Title("Remove"), ControlId(400 + 3 * index), Anchor(ANF_TOP|ANF_RIGHT)]).posX(300).posY(1).width(60).height(22).content({
                            let vm = self.vm.clone();
                            move|_|{
                                vm.removeToDo(id);
                        }})
                    ))
                ))
            )
            ,
            Panel::new([SP::Title("testing title"), ControlId(104), Anchor(ANF_LEFT|ANF_BOTTOM|ANF_RIGHT)]).posX(4).posY(240).width(385).height(300).content(||(
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
                GroupBox::new([SP::Title("group box"), ControlId(107), Anchor(ANF_TOP|ANF_LEFT|ANF_RIGHT)]).posX(5).posY(40).width(300).height(200).content(||(
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
            Label::new([SP::Title(debugInfo), Anchor(ANF_BOTTOM|ANF_LEFTRIGHT), ControlId(115), SP::FontFace(text)]).posX(4).posY(550).width(1024).height(25)
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
