use gloo_utils::{body, document};
use web_sys::Element;

fn component() -> Element {
    let element = document().create_element("div").unwrap();
    element.set_inner_html("hello trunk!");
    element
}

fn main() {
    body().append_child(component().as_ref()).unwrap();
}
