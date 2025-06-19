#![forbid(unsafe_code)]

use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{
  Document, Element, Event, EventTarget, HtmlElement, HtmlInputElement, Node, Storage,
};

// pub mod toggle;

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[inline]
fn remove_local_storage(local_storage: &Storage) {
  local_storage
    .remove_item("theme")
    .expect("Failed to remove theme from local storage");
}

#[inline]
fn set_local_storage(theme: &str, local_storage: &Storage) {
  local_storage
    .set_item("theme", theme)
    .expect("Set theme in local storage")
}

#[inline]
fn get_local_storage(local_storage: &Storage) -> Option<String> {
  local_storage.get_item("theme").unwrap_or(None)
}

#[inline]
fn remove_data_theme(document_element: &Element) {
  document_element
    .remove_attribute("data-theme")
    .expect("Failed to remove data-theme");
}

#[inline]
fn set_data_theme(theme: &str, document_element: &Element) {
  document_element
    .set_attribute("data-theme", theme)
    .expect("Failed to set data-theme");
}

pub fn set_theme() {
  let window = web_sys::window().expect("Could not access window");
  let document = window.document().expect("Could not access window document");
  let document_element = document
    .document_element()
    .expect("Expecting an element on document");
  let local_storage = window
    .local_storage()
    .unwrap_throw()
    .expect("Can't access local storage");

  if let Some(theme) = get_local_storage(&local_storage) {
    set_data_theme(theme.as_ref(), &document_element);
  }

  theme_toggle_listener(&document);
}

pub fn theme_toggle_listener(document: &Document) {
  let check_boxes = document.query_selector_all("[theme-toggle]").unwrap_throw();
  let entries: web_sys::js_sys::Iterator = check_boxes.values();
  console_log!("NodeList length: {:?}", check_boxes.length());

  let theme_toggle_callback = Closure::wrap(Box::new(move |e: Event| {
    console_log!("{:?}", e.current_target().unwrap_throw());

    let input = e
      .current_target()
      .unwrap_throw()
      .dyn_into::<web_sys::HtmlInputElement>()
      .unwrap_throw();

    console_log!("Checked: {:?}", input.checked());
  }) as Box<dyn FnMut(_)>);

  for entry in entries {
    console_log!("{:?}", entry);
    let element = entry
      .unwrap_throw()
      .dyn_into::<web_sys::HtmlInputElement>()
      .unwrap_throw();

    element
      .add_event_listener_with_callback("click", theme_toggle_callback.as_ref().unchecked_ref())
      .unwrap_throw()
  }

  theme_toggle_callback.forget();
}

pub fn theme_toggle() {
  let window = web_sys::window().expect("Could not access window");
  let document = window.document().expect("Could not access window document");
  let document_element = document
    .document_element()
    .expect("Expecting an element on document");
  let local_storage = window
    .local_storage()
    .unwrap_throw()
    .expect("Can't access local storage");

  let check_boxes = document.query_selector_all("[theme-toggle]").unwrap_throw();
  let entries: web_sys::js_sys::Iterator = check_boxes.values();
  // console_log!("NodeList length: {:?}", check_boxes.length());

  let theme = local_storage.clone().get_item("theme").unwrap_or(None);

  let theme_toggle_callback = Closure::wrap(Box::new(move |e: Event| {
    console_log!("{:?}", e.current_target().unwrap_throw());

    let input = e
      .current_target()
      .unwrap_throw()
      .dyn_into::<web_sys::HtmlInputElement>()
      .unwrap_throw();

    if input.checked() {
      document_element
        .set_attribute("data-theme", &input.value())
        .unwrap_throw();
      local_storage
        .set_item("theme", &input.value())
        .unwrap_throw();
    } else {
      document_element
        .remove_attribute("data-theme")
        .unwrap_throw();
      local_storage.remove_item("theme").unwrap_throw();
    }

    console_log!("Checked: {:?}", input.checked());
  }) as Box<dyn FnMut(_)>);

  for entry in entries {
    console_log!("{:?}", entry);
    let element = entry
      .unwrap_throw()
      .dyn_into::<HtmlInputElement>()
      .unwrap_throw();

    if let Some(theme) = theme.as_deref() {
      if element.value() == theme {
        element.set_checked(true);
      }
    }

    element
      .add_event_listener_with_callback("click", theme_toggle_callback.as_ref().unchecked_ref())
      .unwrap_throw()
  }

  theme_toggle_callback.forget();
}

//   log("{input.value():?}");
// }) as Box<dyn FnMut(_)>);

// pub fn theme_toggle() {
//   let window = web_sys::window().expect("Could not access window");
//   let document = window.document().expect("Could not access window document");
//   let toggle_el = document
//     .query_selector("[theme-toggle]")
//     .unwrap_or(None)
//     .map(|element| element.dyn_into::<HtmlInputElement>().unwrap());
//   let document_element = document
//     .document_element()
//     .expect("Expecting an element on document");
//   let local_storage = window
//     .local_storage()
//     .unwrap_throw()
//     .expect("Can't access local storage");

//   if let Some(toggle_el) = toggle_el {
//     let checked = toggle_el.checked();
//     let theme = toggle_el.value();

//     if checked {
//       set_data_theme(theme.as_ref(), &document_element);
//       set_local_storage(theme.as_ref(), &local_storage);
//     } else {
//       remove_data_theme(&document_element);
//       remove_local_storage(&local_storage);
//     }
//   };
// }

// let theme = local_storage.get_item("theme").unwrap_or(None);

// if let Some(theme) = theme {
//   document_element
//     .set_attribute("data-theme", &theme)
//     .expect("Failed to set theme");
// }

// theme = local_storage.getItem(data_key ? data_key : "theme")
// if (local_storage.getItem(data_key ? data_key : "theme")) {
//   document.documentElement.setAttribute("data-theme", theme);
//   if (toggle_el) {
//     [...document.querySelectorAll("[data-toggle-theme]")].forEach((el) => {
//       el.classList.add(toggle_el.getAttribute('data-act-class'))
//     });
//   }
// }
// })();
// if (toggle_el) {
//   [...document.querySelectorAll("[data-toggle-theme]")].forEach((el) => {
//     el.addEventListener("click", function () {
//       var themesList = el.getAttribute('data-toggle-theme');
//       if (themesList) {
//         var themesArray = themesList.split(",");
//         if (document.documentElement.getAttribute('data-theme') == themesArray[0]) {
//           if (themesArray.length == 1) {
//             document.documentElement.removeAttribute("data-theme");
//             local_storage.removeItem(data_key ? data_key : "theme");
//           }else{
//             document.documentElement.setAttribute("data-theme", themesArray[1]);
//             local_storage.setItem(data_key ? data_key : "theme", themesArray[1]);
//           }
//         } else {
//           document.documentElement.setAttribute("data-theme", themesArray[0]);
//           local_storage.setItem(data_key ? data_key : "theme", themesArray[0]);
//         }
//       }
//       [...document.querySelectorAll("[data-toggle-theme]")].forEach((el) => {
//         el.classList.toggle(this.getAttribute('data-act-class'));
//       });
//     });
//   });
// }

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

pub fn theme_toggle() {
  let window = web_sys::window().expect("Could not access window");
  let document = window.document().expect("Could not access window document");
  let check_boxes = document.query_selector_all("[theme-toggle]").unwrap_throw();
  let entries: web_sys::js_sys::Iterator = check_boxes.values();
  // console_log!("NodeList length: {:?}", check_boxes.length());

  let theme_toggle_callback = Closure::wrap(Box::new(move |e: Event| {
    console_log!("{:?}", e.current_target().unwrap_throw());

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

    console_log!("Checked: {:?}", input.checked());
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
      console_log!("{:?}", entry);
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
      console_log!("{:?}", entry);
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
