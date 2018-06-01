extern crate futures;

use futures::prelude::*;
use futures::task::Context;
use std::error::Error;

struct Test {
    x: u32
}


  pub struct Immutable<T> {
    contents: T
  }
  impl<T> Immutable<T> {
    pub fn new(contents: T) -> Immutable<T> {
      Immutable{ contents }
    }
    pub fn get(&self) -> &T {
      &self.contents
    }
  }


struct MyStruct {
  value: Immutable<bool>
}


pub fn main() {
    let x = Immutable::new("test");
    println!("{:?}", x)
}