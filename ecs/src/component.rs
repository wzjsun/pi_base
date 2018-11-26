use std::clone::Clone;

use pi_lib::slab::{Slab};

use ID;
use world::{Thing};

pub enum EventType<'a, T: Component>  {
    ModifyField(<T as Component>::Point, &'a str),
    ModifyIndex(<T as Component>::Point, usize),
    ModifyFieldIndex(<T as Component>::Point, &'a str, usize),
    Create(<T as Component>::Point),
    Delete(<T as Component>::Point)
}

impl<'a, T: Component> Clone for EventType<'a, T>{
    fn clone (&self) -> Self {
        match self {
            EventType::ModifyField(t, ref s) => EventType::ModifyField(t.clone(), s),
            EventType::ModifyIndex(t, ref u) => EventType::ModifyIndex(t.clone(), u.clone()),
            EventType::ModifyFieldIndex(t, ref s, u) => EventType::ModifyFieldIndex(t.clone(), s, u.clone()) ,
            EventType::Create(t) => EventType::Create(t.clone()),
            EventType::Delete(t) => EventType::Delete(t.clone())
        }
    }
}

pub trait ComponentMgr: 'static+Sized {
    fn new() -> Self;
}

pub trait Component: 'static+Sized {
    type Meta;
    type Point: ID + Clone;
    fn meta() -> &'static Self::Meta;
    fn create_point() -> Self::Point;
}

pub struct ComponentGroup<T: Component>{
    components: Slab<T>,
    monitors: Vec<Box<Fn(EventType<T>)>>
}

impl<T: Component> ComponentGroup<T>{
    pub fn new() -> Self{
        ComponentGroup{
            components: Slab::new(),
            monitors: Vec::new()
        }
    }

    pub fn alloc(&mut self, world: xxx) -> (<T as Component>::Point, &mut T){
        let (id, value) = self.components.alloc();
        let mut point = T::create_point();
        point.set_id(id);
        point.set_world(world);
        (point, value)
    }

    pub fn insert(&mut self, component: T, world: xxx) -> <T as Component>::Point{
        let id = self.components.insert(component);
        let mut point = T::create_point();
        point.set_id(id);
        point.set_world(world);
        point
    }

    pub fn try_remove(&mut self, id: usize) -> Option<T>{
        if !self.components.contains(id){
            return None;
        }
        Some(self.components.remove(id))
    }

    pub fn remove(&mut self, id: usize) -> T {
        self.components.remove(id)
    }

    pub fn try_get(&mut self, id: usize) -> Option<&T>{
        self.components.get(id)
    }

    //这是一个非安全方法
    pub fn get(&self, id: usize) -> &T{
        unsafe{ self.components.get_unchecked(id) }
    }

    pub fn try_get_mut(&mut self, id: usize) -> Option<&mut T>{
        self.components.get_mut(id)
    }

    //这是一个非安全方法
    pub fn get_mut(&mut self, id: usize) -> &mut T{
        unsafe{ self.components.get_unchecked_mut(id) }
    }

    pub fn register_moitor(&mut self, monitor: Box<Fn(EventType<T>)>) -> usize{
        self.monitors.push(monitor);
        self.monitors.len() - 1
    }

    //moitor的容器是一个Vec, 其移除的性能并不高， 如果需要频繁的移除， 考虑使用slab
    pub fn unregister_moitor(&mut self, index: usize) -> Option<Box<Fn(EventType<T>)>>{
        if index >= self.monitors.len(){
            None
        }else {
            Some(self.monitors.remove(index))
        }
    }

    pub fn notify_moitor(&self, event: EventType<T>){
        for it in self.monitors.iter(){
            it(event.clone());
        }
    }
}
