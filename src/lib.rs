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

async fn waiting_query(document: &Document, selector: &str) -> Element {
    let mut i = 0;
    loop {
        let element = document.query_selector(selector).unwrap();
        if let Some(element) = element {
            break element;
        }
        sleep(Duration::from_millis(50*i)).await;
        i += 1;
    }
}

struct LoginPageDesc {
    submit_button_selector: &'static str,
    username_input_selector: &'static str,
    password_input_selector: &'static str,
    form_selector: &'static str,
    submit_button_classes: &'static str,
}

const CAS_LOGIN_PAGE_DESC: LoginPageDesc = LoginPageDesc {
    submit_button_selector: "input[name=submit]",
    username_input_selector: "input[name=username]",
    password_input_selector: "input[name=password]",
    form_selector: "form",
    submit_button_classes: "btn btn-block btn-submit",
};

const PARTAGE_LOGIN_PAGE_DESC: LoginPageDesc = LoginPageDesc {
    submit_button_selector: "input[type=submit]",
    username_input_selector: "input[name=username]",
    password_input_selector: "input[name=password]",
    form_selector: "form",
    submit_button_classes: "ZLoginButton DwtButton",
};

async fn get_password(page_desc: &'static LoginPageDesc) {
    // Get document and wait for submit
    log!("Waiting for form to load to get data...");
    let document = window().unwrap().document().unwrap();
    let submit_button = waiting_query(&document, page_desc.submit_button_selector).await;

    // Create an intermediairy submit button
    let second_button = document.create_element("button").unwrap();
    let second_button: HtmlElement = second_button.dyn_into::<HtmlElement>().unwrap();
    second_button.set_inner_text("Login Forever");
    second_button.set_attribute("class", page_desc.submit_button_classes).unwrap();
    submit_button.insert_adjacent_element("afterend", &second_button).unwrap();

    // Remove the first button from the DOM
    submit_button.remove();

    // Get the form
    let form = document.query_selector(page_desc.form_selector).unwrap().unwrap();
    let form: HtmlFormElement = form.dyn_into().unwrap();

    // Read input values on submit
    log!("Waiting for submission...");
    let on_submit = Closure::wrap(Box::new(move |e: Event| {
        e.prevent_default();
        let password_input = document.query_selector(page_desc.password_input_selector).unwrap().unwrap();
        let password = password_input.dyn_into::<HtmlInputElement>().unwrap().value();
        let username_input = document.query_selector(page_desc.username_input_selector).unwrap().unwrap();
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

async fn enter_password(page_desc: &'static LoginPageDesc, username: String, password: String) {
    // Get document and wait for submit
    log!("Waiting for form to load to enter data...");
    let document = window().unwrap().document().unwrap();
    let username_input = waiting_query(&document, page_desc.username_input_selector).await;
    let password_input = document.query_selector(page_desc.password_input_selector).unwrap().unwrap();
    let submit_button = document.query_selector(page_desc.submit_button_selector).unwrap().unwrap();
    let form = document.query_selector(page_desc.form_selector).unwrap().unwrap();

    // Rename the submit button
    submit_button.set_attribute("name", "submit2").unwrap();
    submit_button.set_attribute("value", "Logging in automatically...").unwrap();

    // Set the values
    username_input
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .set_value(&username);
    password_input
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .set_value(&password);

    // Submit the form
    form
        .dyn_into::<HtmlFormElement>()
        .unwrap()
        .submit()
        .unwrap();
}

async fn auto_login(page_desc: &'static LoginPageDesc) {
    match load_data() {
        Some((username, password)) => enter_password(page_desc, username, password).await,
        None => get_password(page_desc).await,
    }
}

#[wasm_bindgen(start)]
pub async fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    log!("Hello, world!");

    // Get the url
    let window = window().unwrap();
    let document = window.document().unwrap();
    let location = document.location().unwrap();
    let url = location.href().unwrap();

    // Run the appropriate code for the page
    match url.as_str() {
        "https://moodle.insa-rouen.fr/login/index.php" => window.location().set_href("https://moodle.insa-rouen.fr/login/index.php?authCAS=CAS").unwrap(),
        "https://dsi.insa-rouen.fr/cas/" => window.location().set_href("https://dsi.insa-rouen.fr/accounts/login/").unwrap(),
        url if url.starts_with("https://partage.insa-rouen.fr/") => auto_login(&PARTAGE_LOGIN_PAGE_DESC).await,
        url if url.starts_with("https://cas.insa-rouen.fr/") => auto_login(&CAS_LOGIN_PAGE_DESC).await,
        url => log!("Unknown url: {}", url),
    }
}
