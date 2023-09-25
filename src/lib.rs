use std::time::Duration;

use wasm_bindgen::prelude::*;
use web_sys::*;
#[macro_use]
mod util;
use util::*;

fn load_data() -> Option<(String, String)> {
    window()
        .unwrap()
        .local_storage()
        .unwrap()
        .unwrap()
        .get_item("insa-auth-rememberer")
        .unwrap()
        .as_ref()
        .and_then(|v| v.split_once('\0'))
        .map(|(u, p)| (u.to_string(), p.to_string()))
}

fn save_data(username: String, password: String) {
    window()
        .unwrap()
        .local_storage()
        .unwrap()
        .unwrap()
        .set_item("insa-auth-rememberer", &format!("{}\0{}", username, password))
        .unwrap()
}

pub async fn get_password() {
    // Get document and wait for submit
    let document = window().unwrap().document().unwrap();
    let submit_button = loop {
        let submit_button = document.query_selector("h2").unwrap();
        if let Some(submit_button) = submit_button {
            break submit_button;
        }
        sleep(Duration::from_millis(100)).await;
    };
    
    // Read input values on submit
    let on_submit = Closure::wrap(Box::new(move || {
        let password_input = document.query_selector("input[name=password]").unwrap().unwrap();
        let password = password_input.dyn_into::<HtmlInputElement>().unwrap().value();
        let username_input = document.query_selector("input[name=username]").unwrap().unwrap();
        let username = username_input.dyn_into::<HtmlInputElement>().unwrap().value();
        log!("Got username {username} and passsword!");
        save_data(username, password);
    }) as Box<dyn FnMut()>);
    submit_button
        .add_event_listener_with_callback("click", on_submit.as_ref().unchecked_ref())
        .unwrap();
    on_submit.forget();
}

#[wasm_bindgen(start)]
pub async fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    // get password from local storage
    let data= load_data();

    if data.is_none() {
        get_password().await;
    }

    log!("Hello World!");
}
