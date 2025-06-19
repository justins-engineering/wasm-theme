use super::log;
use wasm_bindgen::{JsValue, UnwrapThrowExt};
use web_sys::{Element, wasm_bindgen::JsCast};

// fn check_local_storage(data_key: Option<Element>) {

//   let theme = local_storage.getItem(data_key ? data_key : "theme");
//   // if (local_storage.getItem(data_key ? data_key : "theme")) {
//   //   document.documentElement.setAttribute("data-theme", theme);
//   //   if (toggle_el) {
//   //     [...document.querySelectorAll("[data-toggle-theme]")].forEach((el) => {
//   //       el.classList.add(toggle_el.getAttribute('data-act-class'))
//   //     });
//   //   }
//   // }
// }
const SELECTOR: String = "[theme-toggle]";

pub fn theme_toggle(class: Option<String>) {
  let window = web_sys::window().expect("global window does not exists");
  let document = window.document().expect("expecting a document on window");
  let document_element = document
    .document_element()
    .expect("expecting an element on document");
  let local_storage = window
    .local_storage()
    .unwrap_throw()
    .expect("can't access local storage");
  // let toggle_el: Option<Element> = None;
  let data_key = "theme";
  let mut theme: Option<String> = None;

  let toggle_el = document.query_selector("[theme-toggle]").unwrap_or(None);
  // .unwrap_throw();
  // .dyn_into::<web_sys::HtmlElement>()
  // .unwrap();

  // match toggle_el {
  //   Some(e) => println!(),
  //   _ => {}
  // }

  if let Some(toggle_el) = toggle_el {
    data_key = toggle_el.get_attribute("data-key");
    // log(data_key.as_ref().unwrap());
  } else {
    log("No data-key");
  };
  theme = local_storage.get_item("theme").unwrap_or(None);

  if let Some(theme) = theme {
    document_element
      .set_attribute("data-theme", &theme)
      .expect("Failed to set theme");
  }

  // theme = local_storage.getItem(data_key ? data_key : "theme")
  // if (local_storage.getItem(data_key ? data_key : "theme")) {
  //   document.documentElement.setAttribute("data-theme", theme);
  //   if (toggle_el) {
  //     [...document.querySelectorAll("[data-toggle-theme]")].forEach((el) => {
  //       el.classList.add(toggle_el.getAttribute('data-act-class'))
  //     });
  //   }
  // }
}
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
