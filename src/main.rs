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
}

impl MyView {
    pub fn new() -> Self {
        Self {
            vm: MainViewModel::shared(),
        }
    }
}

impl Renderable for MyView {
    fn childs(&self) -> Option<Rc<dyn Renderable>> {
        let showAll: &usize = &self.vm.showAll.borrow();
        let newTitle: &str = &self.vm.newTitle.borrow();
        let allChecked = self.vm.items.borrow().len() == self.vm.getCompleted();

        Some(Rc::from(Window::new([
            SP::ClassName("window#1"), ControlId(0), Anchor(ANF_DOCK_ALL), SP::Title("TODO: Example"), SP::FontFace("Monaco"),
            Width(500), Height(310)
        ]).content(||(
            Panel::new([SP::Title("create new todo"), ControlId(103), Anchor(ANF_DOCK_TOP)]).posX(0).posY(0).width(484).height(35).content(||(
                CheckBox::new([SP::Title("Mark All"), ControlId(102), Anchor(ANF_TOP|ANF_LEFT), Selected(allChecked)]).posX(5).posY(5).width(70).height(25).content(Command::new({
                    let vm = self.vm.clone();
                    move|event: Vec<SharedProps>| {
                        if let Some(checked) = event.prop(&SharedProps::Selected(true)) {
                            vm.completeAll();
                        }
                    }
                }))
                ,
                Label::new([SP::Title("New todo title:")]).posX(76).posY(9).width(120).height(25)
                ,
                TextBox::new([ControlId(201), Anchor(ANF_TOP| ANF_LEFTRIGHT)]).title(newTitle).posX(170).posY(6).width(260).height(21).content({
                    let newTitle = Rc::clone(&self.vm.newTitle);
                    move|event: Vec<SharedProps>|{
                        let mut newTitle = newTitle.borrow_mut();
                        if let Some(Title(v)) = event.prop(&SP::Title("")) {
                            *newTitle = v.to_string();
                        }
                    }
                })
                ,
                Button::new([SP::Title("Save"), ControlId(202), Anchor(ANF_TOP|ANF_RIGHT)]).posX(436).posY(4).width(40).height(24).content({
                    let vm = self.vm.clone();
                    move|_|vm.createToDo()
                })
                ,
            ))
            ,
            Panel::new([ControlId(203), Anchor(ANF_TOPBOTTOM|ANF_LEFTRIGHT)]).posX(4).posY(40).width(475).height(200).content(||
                ForEach::new(self.vm.getItems().iter().map(|i|i.clone()).collect::<Vec<_>>(), |(id, name, isFinished), index|(
                    Panel::new([ControlId(300 + index), Anchor(ANF_LEFT|ANF_TOP|ANF_DOCK_RIGHT_EX)]).posX(4).posY(2 + index * 29).height(27).width(400).content(||(
                        CheckBox::new([Selected(isFinished), SP::Title("Done"), ControlId(400 + index), Anchor(ANF_TOP|ANF_LEFT)]).posX(4).width(50).posY(1).height(24).content({
                            let vm = self.vm.clone();
                            let name = name.clone();
                            move|args: Vec<SharedProps>|{
                                vm.updateToDo((id, name.clone(), !isFinished));
                        }})
                        ,
                        TextBox::new([SP::Title(name.as_str()), ControlId(400 + 2 * index), Anchor(ANF_TOP|ANF_LEFTRIGHT)]).posX(55).posY(1).width(350).height(24).content({
                            let vm = self.vm.clone();
                            move|args: Vec<SharedProps>|{
                                if let Some(Title(title)) = args.prop(&SP::Title("")) {
                                    vm.updateToDo((id, title.to_string(), isFinished));
                                }
                            }
                        })
                        ,
                        Button::new([SP::Title("Remove"), ControlId(400 + 3 * index), Anchor(ANF_TOP|ANF_RIGHT)]).posX(406).posY(1).width(60).height(22).content({
                            let vm = self.vm.clone();
                            move|_|vm.removeToDo(id)
                        })
                        ,
                    ))
                ))
            )
            ,
            Panel::new([SP::Title("testing title"), ControlId(104), Anchor(ANF_DOCK_BOTTOM)]).posX(0).posY(243).width(484).height(28).content(||(
                Label::new([SP::Title(format!("{} item left", self.vm.getCompleted()).as_str()), ControlId(105), Anchor(ANF_TOP|ANF_LEFT), SP::FontFace("Monaco")]).posX(5).posY(5).width(125).height(25)
                ,
                RadioBox::new([SP::Title("All"), ControlId(110), Anchor(ANF_TOP|ANF_LEFT), Selected(*showAll == 0)]).posX(130).posY(2).width(45).height(25).content({
                    let vm = self.vm.clone();
                    move|_|vm.setShowAll(0)
                })
                ,
                RadioBox::new([SP::Title("Active"), ControlId(111), Anchor(ANF_TOP|ANF_LEFT), Selected(*showAll == 1)]).posX(175).posY(2).width(65).height(25).content({
                    let vm = self.vm.clone();
                    move|_|vm.setShowAll(1)
                })
                ,
                RadioBox::new([SP::Title("Completed"), ControlId(111), Anchor(ANF_TOP|ANF_LEFT), Selected(*showAll == 2)]).posX(240).posY(2).width(85).height(25).content({
                    let vm = self.vm.clone();
                    move|_|vm.setShowAll(2)
                })
                ,
                Button::new([SP::Title("Clear Completed"), ControlId(112), Anchor(ANF_TOP|ANF_RIGHT)]).posX(360).posY(2).width(118).height(23).content({
                    let vm = self.vm.clone();
                    move|_|vm.clearCompleted()
                })
                ,
            ))
            ,
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
