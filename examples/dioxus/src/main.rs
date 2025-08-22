use dioxus::prelude::*;
use wasm_theme::{theme_buttons, theme_radio, theme_select, theme_toggle};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
  dioxus::launch(App);
}

#[component]
fn App() -> Element {
  use_effect(move || {
    theme_toggle();
    theme_radio();
    theme_buttons();
    theme_select();
  });

  rsx! {
    document::Link { rel: "icon", href: FAVICON }
    document::Link { rel: "stylesheet", href: TAILWIND_CSS }
    Hero {}
  }
}

#[component]
pub fn Hero() -> Element {
  rsx! {
    div { class: "h-full flex flex-col justify-between",
      img { src: HEADER_SVG }
      div { class: "h-full flex items-center flex-col sm:flex-row justify-around",
        Card { title: "Toggle",
          label { class: "toggle toggle-xl ",
            input {
              r#type: "checkbox",
              name: "theme-toggle",
              value: "light,dark",
            }
            svg {
              "aria-label": "enabled",
              xmlns: "http://www.w3.org/2000/svg",
              view_box: "0 0 24 24",
              g { fill: "currentColor",
                path { d: "M5.64,17l-.71.71a1,1,0,0,0,0,1.41,1,1,0,0,0,1.41,0l.71-.71A1,1,0,0,0,5.64,17ZM5,12a1,1,0,0,0-1-1H3a1,1,0,0,0,0,2H4A1,1,0,0,0,5,12Zm7-7a1,1,0,0,0,1-1V3a1,1,0,0,0-2,0V4A1,1,0,0,0,12,5ZM5.64,7.05a1,1,0,0,0,.7.29,1,1,0,0,0,.71-.29,1,1,0,0,0,0-1.41l-.71-.71A1,1,0,0,0,4.93,6.34Zm12,.29a1,1,0,0,0,.7-.29l.71-.71a1,1,0,1,0-1.41-1.41L17,5.64a1,1,0,0,0,0,1.41A1,1,0,0,0,17.66,7.34ZM21,11H20a1,1,0,0,0,0,2h1a1,1,0,0,0,0-2Zm-9,8a1,1,0,0,0-1,1v1a1,1,0,0,0,2,0V20A1,1,0,0,0,12,19ZM18.36,17A1,1,0,0,0,17,18.36l.71.71a1,1,0,0,0,1.41,0,1,1,0,0,0,0-1.41ZM12,6.5A5.5,5.5,0,1,0,17.5,12,5.51,5.51,0,0,0,12,6.5Zm0,9A3.5,3.5,0,1,1,15.5,12,3.5,3.5,0,0,1,12,15.5Z" }
              }
            }
            svg {
              "aria-label": "disabled",
              xmlns: "http://www.w3.org/2000/svg",
              view_box: "0 0 24 24",
              g { fill: "currentColor",
                path { d: "M21.64,13a1,1,0,0,0-1.05-.14,8.05,8.05,0,0,1-3.37.73A8.15,8.15,0,0,1,9.08,5.49a8.59,8.59,0,0,1,.25-2A1,1,0,0,0,8,2.36,10.14,10.14,0,1,0,22,14.05,1,1,0,0,0,21.64,13Zm-9.5,6.69A8.14,8.14,0,0,1,7.08,5.22v.27A10.15,10.15,0,0,0,17.22,15.63a9.79,9.79,0,0,0,2.1-.22A8.11,8.11,0,0,1,12.14,19.73Z" }
              }
            }
          }
          label { class: "swap swap-rotate",
            input {
              r#type: "checkbox",
              name: "theme-toggle",
              value: "dark",
            }
            svg {
              class: "swap-off h-8 w-8 fill-current",
              xmlns: "http://www.w3.org/2000/svg",
              view_box: "0 0 24 24",
              path { d: "M5.64,17l-.71.71a1,1,0,0,0,0,1.41,1,1,0,0,0,1.41,0l.71-.71A1,1,0,0,0,5.64,17ZM5,12a1,1,0,0,0-1-1H3a1,1,0,0,0,0,2H4A1,1,0,0,0,5,12Zm7-7a1,1,0,0,0,1-1V3a1,1,0,0,0-2,0V4A1,1,0,0,0,12,5ZM5.64,7.05a1,1,0,0,0,.7.29,1,1,0,0,0,.71-.29,1,1,0,0,0,0-1.41l-.71-.71A1,1,0,0,0,4.93,6.34Zm12,.29a1,1,0,0,0,.7-.29l.71-.71a1,1,0,1,0-1.41-1.41L17,5.64a1,1,0,0,0,0,1.41A1,1,0,0,0,17.66,7.34ZM21,11H20a1,1,0,0,0,0,2h1a1,1,0,0,0,0-2Zm-9,8a1,1,0,0,0-1,1v1a1,1,0,0,0,2,0V20A1,1,0,0,0,12,19ZM18.36,17A1,1,0,0,0,17,18.36l.71.71a1,1,0,0,0,1.41,0,1,1,0,0,0,0-1.41ZM12,6.5A5.5,5.5,0,1,0,17.5,12,5.51,5.51,0,0,0,12,6.5Zm0,9A3.5,3.5,0,1,1,15.5,12,3.5,3.5,0,0,1,12,15.5Z" }
            }
            svg {
              class: "swap-on h-8 w-8 fill-current",
              xmlns: "http://www.w3.org/2000/svg",
              view_box: "0 0 24 24",
              path { d: "M21.64,13a1,1,0,0,0-1.05-.14,8.05,8.05,0,0,1-3.37.73A8.15,8.15,0,0,1,9.08,5.49a8.59,8.59,0,0,1,.25-2A1,1,0,0,0,8,2.36,10.14,10.14,0,1,0,22,14.05,1,1,0,0,0,21.64,13Zm-9.5,6.69A8.14,8.14,0,0,1,7.08,5.22v.27A10.15,10.15,0,0,0,17.22,15.63a9.79,9.79,0,0,0,2.1-.22A8.11,8.11,0,0,1,12.14,19.73Z" }
            }
          }
        }
        Card { title: "Radio",
          fieldset { class: "fieldset",
            label { class: "flex gap-2 cursor-pointer items-center",
              input {
                r#type: "radio",
                name: "theme-radios",
                class: "radio radio-sm",
                value: "default",
              }
              "Default"
            }
            label { class: "flex gap-2 cursor-pointer items-center",
              input {
                r#type: "radio",
                name: "theme-radios",
                class: "radio radio-sm",
                value: "light",
              }
              "Light"
            }
            label { class: "flex gap-2 cursor-pointer items-center",
              input {
                r#type: "radio",
                name: "theme-radios",
                class: "radio radio-sm",
                value: "dark",
              }
              "Dark"
            }
            label { class: "flex gap-2 cursor-pointer items-center",
              input {
                r#type: "radio",
                name: "theme-radios",
                class: "radio radio-sm",
                value: "cupcake",
              }
              "Cupcake"
            }
          }
        }
        Card { title: "Buttons",
          button {
            class: "btn btn-primary",
            name: "theme-button",
            value: "light",
            "Light"
          }
          button {
            class: "btn btn-secondary",
            name: "theme-button",
            value: "dark",
            "Dark"
          }
          button {
            class: "btn btn-accent",
            name: "theme-button",
            value: "cupcake",
            "Cupcake"
          }
        }
        Card { title: "Select",
          select { class: "select", name: "theme-select",
            option { value: "default", "Default" }
            option { value: "light", "Light" }
            option { value: "dark", "Dark" }
            option { value: "cupcake", "Cupcake" }
          }
        }
      }
    }
  }
}

#[component]
pub fn Card(title: String, children: Element) -> Element {
  rsx! {
    div { class: "card bg-base-300 text-base-content w-full sm:w-1/5",
      div { class: "card-body items-center text-center",
        h2 { class: "card-title", "{title}" }
        div { class: "card-actions", {&children} }
      }
    }
  }
}
