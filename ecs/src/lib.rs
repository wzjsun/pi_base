extern crate pi_lib;

pub mod world;
pub mod component;
pub mod system;


pub trait ID{
    fn id(&self) -> usize;
    fn set_id(&mut self, id: usize);
}
