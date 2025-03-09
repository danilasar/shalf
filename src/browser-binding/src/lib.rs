mod utils;
mod dom;
mod object_searcher;

use wasm_bindgen::prelude::*;
use js_sys::{Function, Map, Object, Promise, Reflect, Uint8Array, WebAssembly::{self, Module}};
use web_sys::{Request, RequestInit, Response};
use wasm_bindgen_futures::JsFuture;
use std::sync::Mutex;

static DOM: Mutex<dom::DOM> = Mutex::new(dom::DOM::new());

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, {{project-name}}!");
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

fn bind(this: &JsValue, func_name: &str) -> Result<(), JsValue> {
    let property_key = JsValue::from(func_name);
    let orig_func = Reflect::get(this, &property_key)?.dyn_into::<Function>()?;
    let func = orig_func.bind(this);
    if !Reflect::set(this, &property_key, &func)? {
        return Err(JsValue::from("failed to set property"));
    }
    Ok(())
}

pub fn make_imports() -> Result<Object, JsValue> {
    let map = Map::new();
    let imports: JsValue = Imports.into();

    bind(&imports, "native_add")?;
    bind(&imports, "alert")?;
    bind(&imports, "abort")?;
    bind(&imports, "sendnumber")?;

    map.set(&JsValue::from("env"), &imports);
    Object::from_entries(&map.into())
}

#[wasm_bindgen]
pub struct Imports;

#[wasm_bindgen]
impl Imports {
    pub fn native_add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
    pub fn alert(&self, s: &[i8]) {
        //alert(s);
        console_log!("{:#?}", s);
    }
    pub fn sendnumber(&self, s: i32) {
        console_log!("{}", s);
    }
    pub fn abort(&self) {
        console_log!("Хуякс");
    }
}

pub struct BrowserRuntime;

impl shalf_core::Runtime for BrowserRuntime {
    fn get_environment_name(&self) -> String {
        "browser".into()
    }
    fn println(&self, s: &str) {
        console_log!("{}", s);
    }
}

#[wasm_bindgen]
pub async fn load_and_add(url: &str, a: i32, b: i32) -> Result<i32, JsValue> {
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(web_sys::RequestMode::Cors);

    let request = Request::new_with_str_and_init(url, &opts)?;
    request.headers().set("Accept", "application/x-shalf")?;

    let window = web_sys::window().unwrap();
    let response = JsFuture::from(window.fetch_with_request(&request)).await?
        .dyn_into::<Response>()?;

    let buffer = JsFuture::from(response.array_buffer()?).await?;
    let bytes = Uint8Array::new(&buffer).to_vec();

    let module = JsFuture::from(WebAssembly::compile(&buffer)).await?;
    let module= Module::from(module);

    let imports = make_imports()?;
    console_log!("{:#?}", imports);

    let instance = JsFuture::from(WebAssembly::instantiate_module(&module, &imports))
        .await?
        .dyn_into::<WebAssembly::Instance>()?;

    let exports = instance.exports();
    let add = Reflect::get(&exports, &JsValue::from_str("add"))?
        .dyn_into::<js_sys::Function>()?;

    let result = add.call2(&JsValue::NULL, &a.into(), &b.into())?;
    //let result = result.map(|v| v.as_f64().unwrap() as i32)?;
    let result = result.as_f64().map(|v| v as i32).ok_or(0)?;

    return Ok(result);
    Ok(69)
}

#[wasm_bindgen(start)]
pub fn main() {
    shalf_core::main(BrowserRuntime);
    shalf_core::hello_world();
    log("WASM Loaded!");
}
