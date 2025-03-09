use crate::Runtime;

pub struct NoRuntime;
impl Runtime for NoRuntime {
    fn get_environment_name(&self) -> String { "void".into() }
    fn println(&self, _: &str) {}
    fn fetch(&self, _: &str) {}
}
