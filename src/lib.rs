use std::time::Duration;

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
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
    log!("Waiting for form to load...");
    let document = window().unwrap().document().unwrap();
    let submit_button = loop {
        let submit_button = document.query_selector("input[name=submit]").unwrap();
        if let Some(submit_button) = submit_button {
            break submit_button;
        }
        sleep(Duration::from_millis(100)).await;
    };

    // Create an intermediairy submit button
    let second_button = document.create_element("button").unwrap();
    let second_button: HtmlElement = second_button.dyn_into::<HtmlElement>().unwrap();
    second_button.set_inner_text("Login Forever");
    second_button.set_attribute("class", "btn btn-block btn-submit").unwrap();
    submit_button.insert_adjacent_element("afterend", &second_button).unwrap();

    // Remove the first button from the DOM
    submit_button.remove();

    // Get the form
    let form = document.query_selector("form").unwrap().unwrap();
    let form: HtmlFormElement = form.dyn_into().unwrap();

    // Read input values on submit
    log!("Waiting for submission...");
    let on_submit = Closure::wrap(Box::new(move |e: Event| {
        e.prevent_default();
        let password_input = document.query_selector("input[name=password]").unwrap().unwrap();
        let password = password_input.dyn_into::<HtmlInputElement>().unwrap().value();
        let username_input = document.query_selector("input[name=username]").unwrap().unwrap();
        let username = username_input.dyn_into::<HtmlInputElement>().unwrap().value();
        log!("Got username {username} and passsword!");
        save_data(username, password);
        form.submit().unwrap();
    }) as Box<dyn FnMut(Event)>);
    second_button
        .add_event_listener_with_callback("click", on_submit.as_ref().unchecked_ref())
        .unwrap();
    on_submit.forget();
}

#[wasm_bindgen(start)]
pub async fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    log!("Hello World!");

    // get password from local storage
    let data = load_data();

    spawn_local(get_password());

     
    log!("Got password!");
}
