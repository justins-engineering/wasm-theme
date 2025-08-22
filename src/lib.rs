#![forbid(unsafe_code)]
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{Document, Window};

fn prefers_color_scheme(window: Window) -> String {
  let document = window.document().expect("Could not access window document");
  let storage = window
    .local_storage()
    .unwrap_throw()
    .expect("Can't access local storage");
  let theme = storage.get_item("theme").unwrap_or(None);

  let document_element = document
    .document_element()
    .expect("Expecting an element on document");

  let mut data_theme = "default";

  if let Some(theme) = theme.as_deref() {
    data_theme = theme;
  } else if let Ok(Some(scheme)) = window.match_media("(prefers-color-scheme: dark)") {
    if scheme.matches() {
      data_theme = "dark";
      storage.set_item("theme", "dark").unwrap_throw();
    }
  } else if let Ok(Some(scheme)) = window.match_media("(prefers-color-scheme: light)") {
    if scheme.matches() {
      data_theme = "light";
      storage.set_item("theme", "light").unwrap_throw();
    }
  } else {
    storage.set_item("theme", "default").unwrap_throw();
  }

  document_element
    .set_attribute("data-theme", data_theme)
    .expect("Failed to set data-theme");

  data_theme.to_string()
}

fn toggle_callback(window: Window, document: Document) -> Closure<dyn FnMut(web_sys::Event)> {
  Closure::wrap(Box::new(move |e: web_sys::Event| {
    let input = e
      .current_target()
      .unwrap_throw()
      .dyn_into::<web_sys::HtmlInputElement>()
      .unwrap_throw();

    let storage = window
      .local_storage()
      .unwrap_throw()
      .expect("Can't access local storage");
    let document_element = document
      .document_element()
      .expect("Expecting an element on document");

    let value = input.value();
    let mut itr = value.rsplitn(2, ',');
    let checked_value = itr.next();
    let unchecked_value = itr.next();

    if input.checked() {
      if let Some(checked) = checked_value {
        document_element
          .set_attribute("data-theme", checked)
          .unwrap_throw();
        storage.set_item("theme", checked).unwrap_throw();
      } else {
        document_element
          .set_attribute("data-theme", &value)
          .unwrap_throw();
        storage.set_item("theme", &value).unwrap_throw();
      }
    } else if let Some(unchecked) = unchecked_value {
      document_element
        .set_attribute("data-theme", unchecked)
        .unwrap_throw();
      storage.set_item("theme", unchecked).unwrap_throw();
    } else {
      document_element
        .set_attribute("data-theme", "default")
        .unwrap_throw();
      storage.set_item("theme", "default").unwrap_throw();
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
  let callback = toggle_callback(window.clone(), document);
  let prefered = prefers_color_scheme(window);

  for entry in entries {
    let element = entry
      .unwrap_throw()
      .dyn_into::<web_sys::HtmlInputElement>()
      .unwrap_throw();

    let value = element.value();
    let mut itr = value.rsplitn(2, ',');
    let checked_value = itr.next();

    if let Some(checked) = checked_value {
      if checked == prefered {
        element.set_checked(true);
      }
    } else if element.value() == prefered {
      element.set_checked(true);
    }

    element
      .add_event_listener_with_callback("click", callback.as_ref().unchecked_ref())
      .unwrap_throw()
  }

  callback.forget();
}

fn radio_callback(window: Window, document: Document) -> Closure<dyn FnMut(web_sys::Event)> {
  Closure::wrap(Box::new(move |e: web_sys::Event| {
    let input = e
      .current_target()
      .unwrap_throw()
      .dyn_into::<web_sys::HtmlInputElement>()
      .unwrap_throw();
    let storage = window
      .local_storage()
      .unwrap_throw()
      .expect("Can't access local storage");
    let document_element = document
      .document_element()
      .expect("Expecting an element on document");

    document_element
      .set_attribute("data-theme", &input.value())
      .unwrap_throw();
    storage.set_item("theme", &input.value()).unwrap_throw();
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
  let callback = radio_callback(window.clone(), document);
  let prefered = prefers_color_scheme(window);

  for entry in entries {
    let element = entry
      .unwrap_throw()
      .dyn_into::<web_sys::HtmlInputElement>()
      .unwrap_throw();

    if element.value() == prefered {
      element.set_checked(true);
    }

    element
      .add_event_listener_with_callback("click", callback.as_ref().unchecked_ref())
      .unwrap_throw()
  }

  callback.forget();
}

fn button_callback(window: Window, document: Document) -> Closure<dyn FnMut(web_sys::Event)> {
  Closure::wrap(Box::new(move |e: web_sys::Event| {
    let button = e
      .current_target()
      .unwrap_throw()
      .dyn_into::<web_sys::HtmlButtonElement>()
      .unwrap_throw();
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
  let buttons = document
    .query_selector_all("[name=theme-button]")
    .unwrap_throw();
  let entries: web_sys::js_sys::Iterator = buttons.values();
  let callback = button_callback(window.clone(), document);
  let _prefered = prefers_color_scheme(window);

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

fn select_callback(window: Window, document: Document) -> Closure<dyn FnMut(web_sys::Event)> {
  Closure::wrap(Box::new(move |e: web_sys::Event| {
    let select = e
      .current_target()
      .unwrap_throw()
      .dyn_into::<web_sys::HtmlSelectElement>()
      .unwrap_throw();
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
  let callback = select_callback(window.clone(), document);
  let prefered = prefers_color_scheme(window);

  for entry in entries {
    let element = entry
      .unwrap_throw()
      .dyn_into::<web_sys::HtmlSelectElement>()
      .unwrap_throw();

    element.set_value(&prefered);

    element
      .add_event_listener_with_callback("change", callback.as_ref().unchecked_ref())
      .unwrap_throw()
  }

  callback.forget();
}
