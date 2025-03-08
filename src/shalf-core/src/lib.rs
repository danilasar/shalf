use std::sync::Mutex;
use lazy_static::lazy_static;

pub trait Runtime : Send + Sync {
    fn get_environment_name(&self) -> String;
    fn println(&self, s: &str);
}

struct NoRuntime;
impl Runtime for NoRuntime {
    fn get_environment_name(&self) -> String { "void".into() }
    fn println(&self, _: &str) {}
}

lazy_static! {
    static ref RUNTIME: Mutex<Box<dyn Runtime>> = Mutex::new(Box::new(NoRuntime));
}

pub fn hello_world() {
    RUNTIME.lock().unwrap().println("Hello from Shalf core!");
}

pub fn main(runtime: impl Runtime + 'static) {
    *RUNTIME.lock().unwrap() = Box::new(runtime);
}


pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
