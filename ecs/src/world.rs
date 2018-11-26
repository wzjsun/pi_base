use std::rc::{Rc, Weak};
use std::cell::RefCell;

use component::{ComponentMgr};
use system::SystemMgr;

pub type WeakWorld<S, C> = Weak<RefCell<WorldImpl<S, C>>>;
pub struct World<S: SystemMgr, C: ComponentMgr>(pub Rc<RefCell<WorldImpl<S, C>>>);

impl<S: SystemMgr, C: ComponentMgr> World<S, C> {
    pub fn new(s: S, c: C) -> World<S, C> {
        World(Rc::new(RefCell::new(
            WorldImpl{
                component_mgr: c,
                system_mgr: s,
            }
        )))
    }

    pub fn downgrade(&self) -> WeakWorld<S, C>{
        Rc::downgrade(&self.0)
    }
}


pub struct WorldImpl<S: SystemMgr, C: ComponentMgr>{
    pub component_mgr: C,
    pub system_mgr: S,
}

//事物， 包含组件和系统
pub trait Thing<S: SystemMgr, C: ComponentMgr>{
    fn set_world(&mut self, world: WeakWorld<S, C>);
}


pub fn upgrade_world<S: SystemMgr, C: ComponentMgr>(world: &WeakWorld<S, C>) -> Rc<RefCell<WorldImpl<S, C>>>{
    match world.upgrade() {
        Some(w) => w,
        None => panic!("world lost!"),
    }
}