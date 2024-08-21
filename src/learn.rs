#![allow(dead_code, unused)]

use std::any::Any;

pub trait TLearn {
    fn print_name(&self);
    fn as_any(&self) -> &dyn Any;
    fn get_type(&self) -> &str;
}

pub struct Learn<T: Ord + Copy> {
    name: String,
    value: T
}

impl<T: Ord + Copy> Learn<T> {
    pub fn new(name: String, value: T) -> Self {
        Learn{
            name,
            value
        }
    }

    pub fn print_name(&self) {
        println!("My name is {}", &self.name)
    }

    pub fn get_type(&self) -> &str {
        std::any::type_name::<T>()
    }
}


impl<T: 'static + Ord + Copy> TLearn for Learn<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn print_name(&self) {
        self.print_name();
    }

    fn get_type(&self) -> &str {
        self.get_type()
    }
}