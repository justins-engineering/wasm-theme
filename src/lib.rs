#![forbid(unsafe_code)]

use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, UnwrapThrowExt};

fn toggle_callback() -> Closure<dyn FnMut(web_sys::Event)> {
  Closure::wrap(Box::new(move |e: web_sys::Event| {
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
  }) as Box<dyn FnMut(_)>)
}

#[wasm_bindgen]
pub fn theme_toggle() {
  let window = web_sys::window().expect("Could not access window");
  let document = window.document().expect("Could not access window document");
  let check_boxes = document
    .query_selector_all("[name=theme-toggle]")
    .unwrap_throw();
  let entries: web_sys::js_sys::Iterator = check_boxes.values();
  let storage = window
    .local_storage()
    .unwrap_throw()
    .expect("Can't access local storage");
  let document_element = document
    .document_element()
    .expect("Expecting an element on document");
  let theme = storage.get_item("theme").unwrap_or(None);
  let callback = toggle_callback();

  if let Some(theme) = theme.as_deref() {
    document_element
      .set_attribute("data-theme", theme)
      .expect("Failed to set data-theme");

    for entry in entries {
      let element = entry
        .unwrap_throw()
        .dyn_into::<web_sys::HtmlInputElement>()
        .unwrap_throw();

      if element.value() == theme {
        element.set_checked(true);
      }

      element
        .add_event_listener_with_callback("click", callback.as_ref().unchecked_ref())
        .unwrap_throw()
    }
  } else {
    for entry in entries {
      let element = entry
        .unwrap_throw()
        .dyn_into::<web_sys::HtmlInputElement>()
        .unwrap_throw();

      element
        .add_event_listener_with_callback("click", callback.as_ref().unchecked_ref())
        .unwrap_throw()
    }
  }

  callback.forget();
}

fn radio_callback() -> Closure<dyn FnMut(web_sys::Event)> {
  Closure::wrap(Box::new(move |e: web_sys::Event| {
    let input = e
      .current_target()
      .unwrap_throw()
      .dyn_into::<web_sys::HtmlInputElement>()
      .unwrap_throw();
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
  }) as Box<dyn FnMut(_)>)
}

#[wasm_bindgen]
pub fn theme_radio() {
  let window = web_sys::window().expect("Could not access window");
  let document = window.document().expect("Could not access window document");
  let radios = document
    .query_selector_all("[name=theme-radios]")
    .unwrap_throw();
  let entries: web_sys::js_sys::Iterator = radios.values();
  let storage = window
    .local_storage()
    .unwrap_throw()
    .expect("Can't access local storage");
  let document_element = document
    .document_element()
    .expect("Expecting an element on document");
  let theme = storage.get_item("theme").unwrap_or(None);
  let callback = radio_callback();

  if let Some(theme) = theme.as_deref() {
    document_element
      .set_attribute("data-theme", theme)
      .expect("Failed to set data-theme");

    for entry in entries {
      let element = entry
        .unwrap_throw()
        .dyn_into::<web_sys::HtmlInputElement>()
        .unwrap_throw();

      if element.value() == theme {
        element.set_checked(true);
      }

      element
        .add_event_listener_with_callback("click", callback.as_ref().unchecked_ref())
        .unwrap_throw()
    }
  } else {
    for entry in entries {
      let element = entry
        .unwrap_throw()
        .dyn_into::<web_sys::HtmlInputElement>()
        .unwrap_throw();

      element
        .add_event_listener_with_callback("click", callback.as_ref().unchecked_ref())
        .unwrap_throw()
    }
  }

  callback.forget();
}

fn button_callback() -> Closure<dyn FnMut(web_sys::Event)> {
  Closure::wrap(Box::new(move |e: web_sys::Event| {
    let button = e
      .current_target()
      .unwrap_throw()
      .dyn_into::<web_sys::HtmlButtonElement>()
      .unwrap_throw();
    let window = web_sys::window().expect("Could not access window");
    let document = window.document().expect("Could not access window document");
    let storage = window
      .local_storage()
      .unwrap_throw()
      .expect("Can't access local storage");
    let document_element = document
      .document_element()
      .expect("Expecting an element on document");

    document_element
      .set_attribute("data-theme", &button.value())
      .unwrap_throw();
    storage.set_item("theme", &button.value()).unwrap_throw();
  }) as Box<dyn FnMut(_)>)
}

#[wasm_bindgen]
pub fn theme_buttons() {
  let window = web_sys::window().expect("Could not access window");
  let document = window.document().expect("Could not access window document");
  let radios = document
    .query_selector_all("[name=theme-button]")
    .unwrap_throw();
  let entries: web_sys::js_sys::Iterator = radios.values();
  let storage = window
    .local_storage()
    .unwrap_throw()
    .expect("Can't access local storage");
  let document_element = document
    .document_element()
    .expect("Expecting an element on document");
  let theme = storage.get_item("theme").unwrap_or(None);
  let callback = button_callback();

  if let Some(theme) = theme.as_deref() {
    document_element
      .set_attribute("data-theme", theme)
      .expect("Failed to set data-theme");
  }

  for entry in entries {
    let element = entry
      .unwrap_throw()
      .dyn_into::<web_sys::HtmlButtonElement>()
      .unwrap_throw();

    element
      .add_event_listener_with_callback("click", callback.as_ref().unchecked_ref())
      .unwrap_throw()
  }

  callback.forget();
}

fn select_callback() -> Closure<dyn FnMut(web_sys::Event)> {
  Closure::wrap(Box::new(move |e: web_sys::Event| {
    let select = e
      .current_target()
      .unwrap_throw()
      .dyn_into::<web_sys::HtmlSelectElement>()
      .unwrap_throw();
    let window = web_sys::window().expect("Could not access window");
    let document = window.document().expect("Could not access window document");
    let storage = window
      .local_storage()
      .unwrap_throw()
      .expect("Can't access local storage");
    let document_element = document
      .document_element()
      .expect("Expecting an element on document");

    document_element
      .set_attribute("data-theme", &select.value())
      .unwrap_throw();
    storage.set_item("theme", &select.value()).unwrap_throw();
  }) as Box<dyn FnMut(_)>)
}

#[wasm_bindgen]
pub fn theme_select() {
  let window = web_sys::window().expect("Could not access window");
  let document = window.document().expect("Could not access window document");
  let radios = document
    .query_selector_all("[name=theme-select]")
    .unwrap_throw();
  let entries: web_sys::js_sys::Iterator = radios.values();
  let storage = window
    .local_storage()
    .unwrap_throw()
    .expect("Can't access local storage");
  let document_element = document
    .document_element()
    .expect("Expecting an element on document");
  let theme = storage.get_item("theme").unwrap_or(None);
  let callback = select_callback();

  if let Some(theme) = theme.as_deref() {
    document_element
      .set_attribute("data-theme", theme)
      .expect("Failed to set data-theme");

    for entry in entries {
      let element = entry
        .unwrap_throw()
        .dyn_into::<web_sys::HtmlSelectElement>()
        .unwrap_throw();

      element.set_value(theme);

      element
        .add_event_listener_with_callback("change", callback.as_ref().unchecked_ref())
        .unwrap_throw()
    }
  } else {
    for entry in entries {
      let element = entry
        .unwrap_throw()
        .dyn_into::<web_sys::HtmlSelectElement>()
        .unwrap_throw();

      element
        .add_event_listener_with_callback("change", callback.as_ref().unchecked_ref())
        .unwrap_throw()
    }
  }

  callback.forget();
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
