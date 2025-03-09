use wasm_bindgen::{JsValue, UnwrapThrowExt};
use web_sys::{Document, Node, NodeList, Value};

struct Object {

}

impl Object {
    fn search_objects() -> Vec<Object> {
        let document : &Document = crate::DOM.lock().unwrap_throw().get_document();
        let objects : Result<NodeList, JsValue> = document.query_selector_all("obect[type=application/x-shalf-text]");
        if let Err(objects) = objects {
            return vec![];
        }
        let object_nodes = objects.unwrap();
        let objects : Vec<Object> = vec![];
        for object_entry in object_nodes.values() {
            
        }
        objects
    }
}
