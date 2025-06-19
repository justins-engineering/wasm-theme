#![forbid(unsafe_code)]

use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{
  Document, Element, Event, EventTarget, HtmlElement, HtmlInputElement, Node, Storage, Window,
};

// pub mod toggle;

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

macro_rules! window {
  () => {
    web_sys::window().expect("Could not access window")
  };
}

macro_rules! document {
  ($window:expr) => {
    $window
      .document()
      .expect("Could not access window document")
  };
}

macro_rules! document_element {
  ($document:expr) => {
    $document
      .document_element()
      .expect("Expecting an element on document")
  };
}

macro_rules! storage {
  ($window:expr) => {
    $window
      .local_storage()
      .unwrap_throw()
      .expect("Can't access local storage")
  };
}

// pub fn theme_toggle() {
//   let check_boxes = document!(window!())
//     .query_selector_all("[theme-toggle]")
//     .unwrap_throw();
//   let entries: web_sys::js_sys::Iterator = check_boxes.values();

//   let theme_toggle_callback = Closure::wrap(Box::new(move |e: Event| {
//     let input = e
//       .current_target()
//       .unwrap_throw()
//       .dyn_into::<web_sys::HtmlInputElement>()
//       .unwrap_throw();

//     if input.checked() {
//       document_element!(document!(window!()))
//         .set_attribute("data-theme", &input.value())
//         .unwrap_throw();
//       storage!(window!())
//         .set_item("theme", &input.value())
//         .unwrap_throw();
//     } else {
//       document_element!(document!(window!()))
//         .remove_attribute("data-theme")
//         .unwrap_throw();
//       storage!(window!()).remove_item("theme").unwrap_throw();
//     }
//   }) as Box<dyn FnMut(_)>);

//   let theme = storage!(window!()).get_item("theme").unwrap_or(None);

//   if let Some(theme) = theme.as_deref() {
//     document_element!(document!(window!()))
//       .set_attribute("data-theme", theme)
//       .expect("Failed to set data-theme");

//     for entry in entries {
//       let element = entry
//         .unwrap_throw()
//         .dyn_into::<HtmlInputElement>()
//         .unwrap_throw();

//       if element.value() == theme {
//         element.set_checked(true);
//       }

//       element
//         .add_event_listener_with_callback("click", theme_toggle_callback.as_ref().unchecked_ref())
//         .unwrap_throw()
//     }
//   } else {
//     for entry in entries {
//       console_log!("{:?}", entry);
//       let element = entry
//         .unwrap_throw()
//         .dyn_into::<HtmlInputElement>()
//         .unwrap_throw();

//       element
//         .add_event_listener_with_callback("click", theme_toggle_callback.as_ref().unchecked_ref())
//         .unwrap_throw()
//     }
//   }

//   theme_toggle_callback.forget();
// }

pub fn theme_toggle() {
  let window = web_sys::window().expect("Could not access window");
  let document = window.document().expect("Could not access window document");
  let check_boxes = document.query_selector_all("[theme-toggle]").unwrap_throw();
  let entries: web_sys::js_sys::Iterator = check_boxes.values();

  let theme_toggle_callback = Closure::wrap(Box::new(move |e: Event| {
    let input = e
      .current_target()
      .unwrap_throw()
      .dyn_into::<web_sys::HtmlInputElement>()
      .unwrap_throw();

    // let window = window!();
    // let local_storage = storage!(window);
    // let document_element = document_element!(document!(window));
    let window = web_sys::window().expect("Could not access window");
    let document = window.document().expect("Could not access window document");
    let storage = window
      .local_storage()
      .unwrap_throw()
      .expect("Can't access local storage");
    let document_element = document
      .document_element()
      .expect("Expecting an element on document");

    if input.checked() {
      document_element
        .set_attribute("data-theme", &input.value())
        .unwrap_throw();
      storage.set_item("theme", &input.value()).unwrap_throw();
    } else {
      document_element
        .remove_attribute("data-theme")
        .unwrap_throw();
      storage.remove_item("theme").unwrap_throw();
    }
  }) as Box<dyn FnMut(_)>);

  let window = web_sys::window().expect("Could not access window");
  let document = window.document().expect("Could not access window document");
  let storage = window
    .local_storage()
    .unwrap_throw()
    .expect("Can't access local storage");
  let document_element = document
    .document_element()
    .expect("Expecting an element on document");

  let theme = storage.get_item("theme").unwrap_or(None);

  if let Some(theme) = theme.as_deref() {
    document_element
      .set_attribute("data-theme", theme)
      .expect("Failed to set data-theme");

    for entry in entries {
      let element = entry
        .unwrap_throw()
        .dyn_into::<HtmlInputElement>()
        .unwrap_throw();

      if element.value() == theme {
        element.set_checked(true);
      }

      element
        .add_event_listener_with_callback("click", theme_toggle_callback.as_ref().unchecked_ref())
        .unwrap_throw()
    }
  } else {
    for entry in entries {
      let element = entry
        .unwrap_throw()
        .dyn_into::<HtmlInputElement>()
        .unwrap_throw();

      element
        .add_event_listener_with_callback("click", theme_toggle_callback.as_ref().unchecked_ref())
        .unwrap_throw()
    }
  }

  theme_toggle_callback.forget();
}

#[wasm_bindgen]
extern "C" {
  // Use `js_namespace` here to bind `console.log(..)` instead of just
  // `log(..)`
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);

  // The `console.log` is quite polymorphic, so we can bind it with multiple
  // signatures. Note that we need to use `js_name` to ensure we always call
  // `log` in JS.
  #[wasm_bindgen(js_namespace = console, js_name = log)]
  fn log_u32(a: u32);

  // Multiple arguments too!
  #[wasm_bindgen(js_namespace = console, js_name = log)]
  fn log_many(a: &str, b: &str);
}

// #[cfg(test)]
// mod tests {
//   use super::*;

//   #[test]
//   fn it_works() {
//     let result = add(2, 2);
//     assert_eq!(result, 4);
//   }
// }
