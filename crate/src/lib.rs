#![feature(once_cell)] // 1.53.0-nightly (2021-04-01 d474075a8f28ae9a410e)
#[macro_use]
extern crate cfg_if;
extern crate regex;
extern crate wasm_bindgen;
extern crate web_sys;
extern crate js_sys;

use std::sync::atomic::{Ordering};
use std::time::Duration;
use std::{lazy::SyncLazy, sync::RwLock};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlCollection, HtmlElement, MutationObserver, MutationObserverInit};

mod structs;
use structs::State;

cfg_if! {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        use console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        fn set_panic_hook() {}
    }
}

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

static STATE: SyncLazy<RwLock<State>> = SyncLazy::new(|| RwLock::new(State::new()));

// Called by our JS entry point to run the example
#[wasm_bindgen]
pub fn start() -> Result<(), JsValue> {
    // If the `console_error_panic_hook` feature is enabled this will set a panic hook, otherwise
    // it will do nothing.
    set_panic_hook();
    //web_sys::console::log_1(&JsValue::from_str("in start"));

    Ok(())
}

#[wasm_bindgen]
pub fn update_letter(letter: char) -> Result<(), JsValue> {
    STATE.write().unwrap().letter = letter;
    Ok(())
}

#[wasm_bindgen]
pub fn update_text() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    //web_sys::console::log_1(&JsValue::from_str("in update text"));
    
    let mut observer_config = MutationObserverInit::new();
    observer_config.character_data(true);
    observer_config.child_list(true);
    observer_config.subtree(true);
    
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let func = js_sys::Function::new_no_args("on_body_change();");
    let func = Closure::wrap(Box::new(move || { on_body_change();}) as Box<dyn FnMut()>);
    let observer = MutationObserver::new(func.as_ref().unchecked_ref()).unwrap();
    func.forget();
    let _ = observer.observe_with_options(&body, &observer_config);

    let _ = on_body_change();

    Ok(())
}

#[wasm_bindgen]
pub fn on_body_change() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    update_html_element(&body);

    Ok(())
}

pub fn update_html_element(e: &HtmlElement) {
    //web_sys::console::log_1(&JsValue::from_str(e.tag_name().as_str()));
    let classes1 = e.class_list();
    if classes1.contains("Header-Social-Follow") || e.tag_name().contains("script"){
        return;
    }
    classes1.add_1("test");
    if e.child_element_count() > 0 {
        let children: HtmlCollection = e.children();
        for i in 0..children.length() {
            let e1: Element = children.get_with_index(i).unwrap();
            let e2: JsValue = e1.into();
            let e3 = e2.into();
            update_html_element(&e3)
        }
    } else {
        let classes = e.class_list();
        let inner_text: String = e.inner_text();
        if inner_text.is_empty(){
            return;
        }
        //web_sys::console::log_1(&JsValue::from_str(inner_text.as_str()));
        if inner_text.len() > 1 && classes.contains("redact") {
            classes.remove_1("redact");
        }
        if !classes.contains("redact") {
            let new_text = inner_text.chars().fold(String::from(""), |mut acc, x| {
                if x.is_whitespace() {
                    acc.push(x);
                    acc
                } else {
                    let num = STATE.read().unwrap().atomic_id.fetch_add(1, Ordering::AcqRel);
                    //https://starbeamrainbowlabs.com/blog/article.php?article=posts%2F415-pure-css-spoiler.html
                    let id = format!("<a class='redact' id='redactId-{num}' href='#redactId-{num}'>{x}</a>", num=num, x=x);
                    acc.push_str(&id);
                    acc
                }
            });

            e.set_inner_html(&new_text);
        }
    } 
}
