//! Set the `data-theme` attribute on the document's root `<html>` element
//! from checkbox/toggle, radio-button, button, and/or `<select>` controls,
//! persisting the choice to `localStorage` and falling back to the
//! browser's `prefers-color-scheme` on first load. See the crate
//! [README](https://github.com/justins-engineering/wasm-theme) for usage
//! with each control type, and for wiring into tailwindcss/daisyUI.
#![forbid(unsafe_code)]
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{Document, Storage, Window};

/// Marks an element whose listener has already been attached, so a
/// repeated call to one of the `theme_*` functions below (e.g. a
/// `use_effect` that reruns, or a component that remounts) doesn't attach
/// a second listener to the same element.
const BOUND_ATTR: &str = "data-wasm-theme-bound";

thread_local! {
  // Memoizes the theme resolved by `prefers_color_scheme` for the page's
  // lifetime. `theme_toggle`/`theme_radio`/`theme_buttons`/`theme_select`
  // are commonly all called together from a single `use_effect` (see the
  // README) — without this, each call would redundantly re-read local
  // storage, re-evaluate `matchMedia`, and re-write the same `data-theme`
  // attribute and storage entry.
  static CACHED_THEME: RefCell<Option<String>> = const { RefCell::new(None) };
}

/// `local_storage()` returns `Err` when the browser denies access — e.g. a
/// sandboxed iframe without `allow-same-origin`, some private browsing
/// modes, or a "block all cookies/storage" setting. Treat that the same as
/// "nothing stored yet" rather than panicking: the theme still applies for
/// the current page load, it just won't survive a reload.
fn local_storage(window: &Window) -> Option<Storage> {
  window.local_storage().ok().flatten()
}

/// Best-effort persist `theme` to local storage; logs and gives up rather
/// than panicking if storage is unavailable or the write fails (e.g. quota
/// exceeded).
fn store_theme(storage: Option<&Storage>, theme: &str) {
  let Some(storage) = storage else { return };
  if storage.set_item("theme", theme).is_err() {
    web_sys::console::error_1(&JsValue::from_str(
      "wasm-theme: failed to persist theme to local storage",
    ));
  }
}

/// Best-effort set `data-theme` on the document's root element.
fn set_data_theme(document: &Document, theme: &str) {
  let Some(document_element) = document.document_element() else {
    return;
  };
  if document_element.set_attribute("data-theme", theme).is_err() {
    web_sys::console::error_1(&JsValue::from_str(
      "wasm-theme: failed to set data-theme attribute",
    ));
  }
}

/// Resolves the theme to apply on load — whatever is already in local
/// storage, else `"dark"` if the browser reports
/// `prefers-color-scheme: dark`, else `"default"` — applies it to the
/// `<html>` element, persists it, and caches the result for the rest of
/// the page's lifetime.
fn prefers_color_scheme(window: &Window, document: &Document) -> String {
  if let Some(cached) = CACHED_THEME.with_borrow(Clone::clone) {
    return cached;
  }

  let storage = local_storage(window);
  let stored = storage
    .as_ref()
    .and_then(|storage| storage.get_item("theme").ok().flatten());

  let data_theme = stored.unwrap_or_else(|| {
    let prefers_dark = window
      .match_media("(prefers-color-scheme: dark)")
      .ok()
      .flatten()
      .is_some_and(|scheme| scheme.matches());
    let theme = if prefers_dark { "dark" } else { "default" }.to_string();
    store_theme(storage.as_ref(), &theme);
    theme
  });

  set_data_theme(document, &data_theme);
  CACHED_THEME.with_borrow_mut(|cached| *cached = Some(data_theme.clone()));
  data_theme
}

/// The value on the "checked" side of a toggle's `value` attribute, i.e.
/// everything after the last comma (or the whole value, if there's no
/// comma).
fn checked_value(value: &str) -> &str {
  value.rsplit(',').next().unwrap_or(value)
}

/// Resolves which theme a toggle's `click` should apply, given its
/// `value` attribute (`"unchecked,checked"`, or a single value paired
/// implicitly with `"default"`) and its new checked state.
fn toggle_theme(value: &str, checked: bool) -> &str {
  let mut itr = value.rsplitn(2, ',');
  let checked_value = itr.next();
  let unchecked_value = itr.next();

  if checked {
    checked_value.unwrap_or(value)
  } else {
    unchecked_value.unwrap_or("default")
  }
}

fn toggle_callback(window: Window, document: Document) -> Closure<dyn FnMut(web_sys::Event)> {
  Closure::wrap(Box::new(move |e: web_sys::Event| {
    let Some(input) = e
      .current_target()
      .and_then(|target| target.dyn_into::<web_sys::HtmlInputElement>().ok())
    else {
      return;
    };

    let value = input.value();
    let theme = toggle_theme(&value, input.checked());

    set_data_theme(&document, theme);
    store_theme(local_storage(&window).as_ref(), theme);
    CACHED_THEME.with_borrow_mut(|cached| *cached = Some(theme.to_string()));
  }) as Box<dyn FnMut(_)>)
}

/// Wires up every `[name=theme-toggle]` checkbox on the page: applies the
/// resolved theme (see [`prefers_color_scheme`]) as each control's initial
/// `checked` state, and attaches a `click` listener that flips
/// `data-theme` between the control's declared checked/unchecked values
/// (`value="light,dark"`) — or between its single `value` and `"default"`
/// if no comma is present.
///
/// Calling this more than once (e.g. a rerun `use_effect`) is safe: a
/// control that's already wired up is left alone rather than gaining a
/// duplicate listener.
///
/// # Panics
///
/// Panics if called outside a browser main-thread context (no `window`, or
/// no `document` on it).
#[wasm_bindgen]
pub fn theme_toggle() {
  let window = web_sys::window().expect_throw("Could not access window");
  let document = window
    .document()
    .expect_throw("Could not access window document");
  let prefered = prefers_color_scheme(&window, &document);

  let Ok(check_boxes) = document.query_selector_all("[name=theme-toggle]") else {
    return;
  };

  let mut callback: Option<Closure<dyn FnMut(web_sys::Event)>> = None;

  for entry in check_boxes.values() {
    let Ok(node) = entry else { continue };
    let Ok(element) = node.dyn_into::<web_sys::HtmlInputElement>() else {
      continue;
    };

    if element.has_attribute(BOUND_ATTR) {
      continue;
    }

    let value = element.value();
    if checked_value(&value) == prefered {
      element.set_checked(true);
    }

    let callback =
      callback.get_or_insert_with(|| toggle_callback(window.clone(), document.clone()));

    if element
      .add_event_listener_with_callback("click", callback.as_ref().unchecked_ref())
      .is_ok()
    {
      let _ = element.set_attribute(BOUND_ATTR, "");
    }
  }

  if let Some(callback) = callback {
    callback.forget();
  }
}

fn radio_callback(window: Window, document: Document) -> Closure<dyn FnMut(web_sys::Event)> {
  Closure::wrap(Box::new(move |e: web_sys::Event| {
    let Some(input) = e
      .current_target()
      .and_then(|target| target.dyn_into::<web_sys::HtmlInputElement>().ok())
    else {
      return;
    };

    let theme = input.value();
    set_data_theme(&document, &theme);
    store_theme(local_storage(&window).as_ref(), &theme);
    CACHED_THEME.with_borrow_mut(|cached| *cached = Some(theme));
  }) as Box<dyn FnMut(_)>)
}

/// Wires up every `[name=theme-radios]` radio button on the page: checks
/// the one whose `value` matches the resolved theme (see
/// [`prefers_color_scheme`]), and attaches a `click` listener that sets
/// `data-theme` to the checked control's `value`.
///
/// Calling this more than once (e.g. a rerun `use_effect`) is safe: a
/// control that's already wired up is left alone rather than gaining a
/// duplicate listener.
///
/// # Panics
///
/// Panics if called outside a browser main-thread context (no `window`, or
/// no `document` on it).
#[wasm_bindgen]
pub fn theme_radio() {
  let window = web_sys::window().expect_throw("Could not access window");
  let document = window
    .document()
    .expect_throw("Could not access window document");
  let prefered = prefers_color_scheme(&window, &document);

  let Ok(radios) = document.query_selector_all("[name=theme-radios]") else {
    return;
  };

  let mut callback: Option<Closure<dyn FnMut(web_sys::Event)>> = None;

  for entry in radios.values() {
    let Ok(node) = entry else { continue };
    let Ok(element) = node.dyn_into::<web_sys::HtmlInputElement>() else {
      continue;
    };

    if element.has_attribute(BOUND_ATTR) {
      continue;
    }

    if element.value() == prefered {
      element.set_checked(true);
    }

    let callback = callback.get_or_insert_with(|| radio_callback(window.clone(), document.clone()));

    if element
      .add_event_listener_with_callback("click", callback.as_ref().unchecked_ref())
      .is_ok()
    {
      let _ = element.set_attribute(BOUND_ATTR, "");
    }
  }

  if let Some(callback) = callback {
    callback.forget();
  }
}

fn button_callback(window: Window, document: Document) -> Closure<dyn FnMut(web_sys::Event)> {
  Closure::wrap(Box::new(move |e: web_sys::Event| {
    let Some(button) = e
      .current_target()
      .and_then(|target| target.dyn_into::<web_sys::HtmlButtonElement>().ok())
    else {
      return;
    };

    let theme = button.value();
    set_data_theme(&document, &theme);
    store_theme(local_storage(&window).as_ref(), &theme);
    CACHED_THEME.with_borrow_mut(|cached| *cached = Some(theme));
  }) as Box<dyn FnMut(_)>)
}

/// Wires up every `[name=theme-button]` button on the page: attaches a
/// `click` listener that sets `data-theme` to the clicked button's
/// `value`.
///
/// Calling this more than once (e.g. a rerun `use_effect`) is safe: a
/// button that's already wired up is left alone rather than gaining a
/// duplicate listener.
///
/// # Panics
///
/// Panics if called outside a browser main-thread context (no `window`, or
/// no `document` on it).
#[wasm_bindgen]
pub fn theme_buttons() {
  let window = web_sys::window().expect_throw("Could not access window");
  let document = window
    .document()
    .expect_throw("Could not access window document");
  // Buttons have no "current" state to compare against the resolved
  // theme, but resolving it still applies+persists the initial theme as a
  // side effect (see `prefers_color_scheme`).
  let _prefered = prefers_color_scheme(&window, &document);

  let Ok(buttons) = document.query_selector_all("[name=theme-button]") else {
    return;
  };

  let mut callback: Option<Closure<dyn FnMut(web_sys::Event)>> = None;

  for entry in buttons.values() {
    let Ok(node) = entry else { continue };
    let Ok(element) = node.dyn_into::<web_sys::HtmlButtonElement>() else {
      continue;
    };

    if element.has_attribute(BOUND_ATTR) {
      continue;
    }

    let callback =
      callback.get_or_insert_with(|| button_callback(window.clone(), document.clone()));

    if element
      .add_event_listener_with_callback("click", callback.as_ref().unchecked_ref())
      .is_ok()
    {
      let _ = element.set_attribute(BOUND_ATTR, "");
    }
  }

  if let Some(callback) = callback {
    callback.forget();
  }
}

fn select_callback(window: Window, document: Document) -> Closure<dyn FnMut(web_sys::Event)> {
  Closure::wrap(Box::new(move |e: web_sys::Event| {
    let Some(select) = e
      .current_target()
      .and_then(|target| target.dyn_into::<web_sys::HtmlSelectElement>().ok())
    else {
      return;
    };

    let theme = select.value();
    set_data_theme(&document, &theme);
    store_theme(local_storage(&window).as_ref(), &theme);
    CACHED_THEME.with_borrow_mut(|cached| *cached = Some(theme));
  }) as Box<dyn FnMut(_)>)
}

/// Wires up every `[name=theme-select]` `<select>` on the page: sets its
/// value to the resolved theme (see [`prefers_color_scheme`]), and
/// attaches a `change` listener that sets `data-theme` to the newly
/// selected `value`.
///
/// Calling this more than once (e.g. a rerun `use_effect`) is safe: a
/// control that's already wired up is left alone rather than gaining a
/// duplicate listener.
///
/// # Panics
///
/// Panics if called outside a browser main-thread context (no `window`, or
/// no `document` on it).
#[wasm_bindgen]
pub fn theme_select() {
  let window = web_sys::window().expect_throw("Could not access window");
  let document = window
    .document()
    .expect_throw("Could not access window document");
  let prefered = prefers_color_scheme(&window, &document);

  let Ok(selects) = document.query_selector_all("[name=theme-select]") else {
    return;
  };

  let mut callback: Option<Closure<dyn FnMut(web_sys::Event)>> = None;

  for entry in selects.values() {
    let Ok(node) = entry else { continue };
    let Ok(element) = node.dyn_into::<web_sys::HtmlSelectElement>() else {
      continue;
    };

    if element.has_attribute(BOUND_ATTR) {
      continue;
    }

    element.set_value(&prefered);

    let callback =
      callback.get_or_insert_with(|| select_callback(window.clone(), document.clone()));

    if element
      .add_event_listener_with_callback("change", callback.as_ref().unchecked_ref())
      .is_ok()
    {
      let _ = element.set_attribute(BOUND_ATTR, "");
    }
  }

  if let Some(callback) = callback {
    callback.forget();
  }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
  use super::*;

  #[test]
  fn toggle_theme_uses_checked_side_when_checked() {
    assert_eq!(toggle_theme("light,dark", true), "dark");
  }

  #[test]
  fn toggle_theme_uses_unchecked_side_when_unchecked() {
    assert_eq!(toggle_theme("light,dark", false), "light");
  }

  #[test]
  fn toggle_theme_falls_back_to_default_when_unchecked_with_no_comma() {
    assert_eq!(toggle_theme("dark", false), "default");
  }

  #[test]
  fn toggle_theme_uses_whole_value_when_checked_with_no_comma() {
    assert_eq!(toggle_theme("dark", true), "dark");
  }

  #[test]
  fn checked_value_takes_last_comma_segment() {
    assert_eq!(checked_value("light,dark"), "dark");
  }

  #[test]
  fn checked_value_is_whole_value_with_no_comma() {
    assert_eq!(checked_value("dark"), "dark");
  }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod browser_tests {
  use super::*;
  use wasm_bindgen_test::*;

  wasm_bindgen_test_configure!(run_in_browser);

  fn window_and_document() -> (Window, Document) {
    let window = web_sys::window().unwrap_throw();
    let document = window.document().unwrap_throw();
    (window, document)
  }

  /// Gets (creating if needed) a `<div>` reserved for test fixtures. Tests
  /// must append their elements here rather than to `<body>` directly and
  /// must never touch `<body>`'s own contents: the wasm-bindgen-test
  /// harness keeps its own tracking elements there too, and clearing all
  /// of `<body>` out from under it silently breaks test-completion
  /// detection (the runner just hangs and reports a timeout, with no
  /// indication the harness's own DOM got wiped).
  fn fixture_container(document: &Document) -> web_sys::Element {
    if let Some(el) = document.get_element_by_id("wasm-theme-test-fixtures") {
      return el;
    }
    let el = document.create_element("div").unwrap_throw();
    el.set_attribute("id", "wasm-theme-test-fixtures")
      .unwrap_throw();
    document
      .body()
      .unwrap_throw()
      .append_child(&el)
      .unwrap_throw();
    el
  }

  fn reset() {
    let (window, document) = window_and_document();
    if let Some(storage) = local_storage(&window) {
      let _ = storage.clear();
    }
    fixture_container(&document).set_inner_html("");
    CACHED_THEME.with_borrow_mut(|cached| *cached = None);
  }

  fn append_toggle(document: &Document, value: &str) -> web_sys::HtmlInputElement {
    let element = document
      .create_element("input")
      .unwrap_throw()
      .dyn_into::<web_sys::HtmlInputElement>()
      .unwrap_throw();
    element.set_type("checkbox");
    element.set_attribute("name", "theme-toggle").unwrap_throw();
    element.set_value(value);
    fixture_container(document)
      .append_child(&element)
      .unwrap_throw();
    element
  }

  #[wasm_bindgen_test]
  fn applies_stored_theme_as_initial_checked_state() {
    reset();
    let (window, document) = window_and_document();
    local_storage(&window)
      .unwrap_throw()
      .set_item("theme", "dark")
      .unwrap_throw();
    let toggle = append_toggle(&document, "light,dark");

    theme_toggle();

    assert!(toggle.checked());
    assert_eq!(
      document
        .document_element()
        .unwrap_throw()
        .get_attribute("data-theme")
        .as_deref(),
      Some("dark")
    );
  }

  #[wasm_bindgen_test]
  fn click_flips_data_theme_and_persists_it() {
    reset();
    let (window, document) = window_and_document();
    let toggle = append_toggle(&document, "light,dark");

    theme_toggle();
    toggle.set_checked(true);
    toggle
      .dispatch_event(&web_sys::Event::new("click").unwrap_throw())
      .unwrap_throw();

    assert_eq!(
      document
        .document_element()
        .unwrap_throw()
        .get_attribute("data-theme")
        .as_deref(),
      Some("dark")
    );
    assert_eq!(
      local_storage(&window)
        .unwrap_throw()
        .get_item("theme")
        .unwrap_throw()
        .as_deref(),
      Some("dark")
    );
  }

  #[wasm_bindgen_test]
  fn second_call_skips_already_bound_elements() {
    reset();
    let (window, document) = window_and_document();
    local_storage(&window)
      .unwrap_throw()
      .set_item("theme", "dark")
      .unwrap_throw();
    let toggle = append_toggle(&document, "light,dark");

    // Pre-mark the element as already bound, and pre-set a checked state
    // that does *not* match the stored theme. A first-time call would
    // flip it to match "dark" (see the test above) — this asserts that an
    // element already marked bound is left alone instead.
    toggle.set_attribute(BOUND_ATTR, "").unwrap_throw();
    toggle.set_checked(false);

    theme_toggle();

    assert!(!toggle.checked());
  }
}
