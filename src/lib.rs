mod utils;
// use js_sys;
use wasm_bindgen::prelude::*;
use js_sys;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// #[wasm_bindgen]
// extern {
//     fn alert(s: &str);
// }

#[wasm_bindgen]
pub fn greet() -> u32 {
    // alert("Hello, hello-wasm!");
    // println!("hello world");
    6
}

#[wasm_bindgen]
pub fn map_obj(o: &js_sys::Object) -> u32 {

5
    // Ok(5)
}

#[wasm_bindgen]
pub fn collect_numbers(some_iterable: &js_sys::Object) -> Result<js_sys::Object, JsValue> {
    // let nums = js_sys::Array::new();
    let obj = js_sys::Object::new();

    // let iterator = js_sys::try_iter(some_iterable)?.ok_or_else(|| {
    //     "need to pass iterable JS values!"
    // })?;


    for name in js_sys::Object::keys(some_iterable).iter() {
        let entry = js_sys::Reflect::get(some_iterable, &name)?;
        let val = js_sys::Reflect::get(&entry, &js_sys::JsString::from("propA"))?;
        js_sys::Reflect::set(&obj, &name, &val);
        // If the iterator's `next` method throws an error, propagate it
        // up to the caller.
        // let x = x?;

        // If `x` is a number, add it to our array of numbers!
        // if x.as_f64().is_some() {
        //     nums.push(&x);
        // }
    }

    Ok(obj)
}