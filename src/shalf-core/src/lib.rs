pub mod parser;
pub mod app;
pub mod prelude;

use std::sync::Mutex;
use lazy_static::lazy_static;

pub trait Runtime : Send + Sync {
    fn get_environment_name(&self) -> String;
    fn println(&self, s: &str);
    fn fetch(&self, url: &str);
}


lazy_static! {
    static ref RUNTIME: Mutex<Box<dyn Runtime>> = Mutex::new(Box::new(prelude::no_runtime::NoRuntime));
}

pub fn main(runtime: impl Runtime + 'static) {
    *RUNTIME.lock().unwrap() = Box::new(runtime);
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn it_works() {
    }
}
