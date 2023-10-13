use js_sys::{Promise, Function, Array};
use wasm_bindgen::{JsValue, JsCast};
use std::{time::Duration, pin::Pin};
use wasm_bindgen_futures::JsFuture;
use web_sys::window;

pub async fn sleep(duration: Duration) {
    JsFuture::from(Promise::new(&mut |yes, _| {
        window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                &yes,
                duration.as_millis() as i32,
            )
            .unwrap();
    })).await.unwrap();
}

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into())
    }
}

pub trait SimpleSerde {
    fn serialize(&self) -> String;
    fn deserialize(s: String) -> Self;
}

pub type FutFn<Input> = Pin<Box<dyn (Fn(Input) -> JsFuture)>>;

pub fn js_function<Input: SimpleSerde>(f: JsValue) -> FutFn<Input> {
    let function: Function = f.dyn_into().unwrap();

    Box::pin(move |input: Input| -> JsFuture {
        let args: Array = Array::new();
        args.push(&input.serialize().into());
        let promise = js_sys::Reflect::apply(&function, &JsValue::UNDEFINED, &args).unwrap();
        let promise: Promise = promise.dyn_into().unwrap();
        wasm_bindgen_futures::JsFuture::from(promise)
    })
}
