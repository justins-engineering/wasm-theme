# WASM Theme
A simple rust library to set CSS `data-theme` on the `html` element with WASM. Compatible with tailwindcss and daisyUI. The user's preferred `data-theme` is saved in local storage as the `theme` variable.

It respects a browsers `prefers-color-scheme` if sent and no `theme` variable is set in local storage.

Support for checkbox/toggle, radio-buttons, buttons, and/or select.

## Using with tailwindcss
```css
@import "tailwindcss";

@custom-variant dark (&:where([data-theme=dark], [data-theme=dark] *));
```

## Using with daisyUI
```css
@import "tailwindcss";

@plugin "daisyui" {
  themes: light --default, dark --prefersdark, cupcake;
};
```

## Dioxus Usage

### Checkbox/toggle
```rust
use wasm_theme::theme_toggle;

#[component]
fn App() -> Element {
  use_effect(move || {
    theme_toggle();
  });

  rsx! {
    ...
    SomeComponent {}
  }
}

#[component]
pub fn SomeComponent() -> Element {
  input {
    r#type: "checkbox",
    name: "theme-toggle",
    // unchecked value will be `light`, checked value will be `dark`
    value: "light,dark",
    // or `value: "dark",` if you want the unchecked value to be `default`
  }
}
```

### Radio buttons
```rust
use wasm_theme::theme_radio;

#[component]
fn App() -> Element {
  use_effect(move || {
    theme_radio();
  });

  rsx! {
    ...
    SomeComponent {}
  }
}

#[component]
pub fn SomeComponent() -> Element {
  label {
    input {
      r#type: "radio",
      name: "theme-radios",
      value: "default",
    }
    "Default"
  }
  label {
    input {
      r#type: "radio",
      name: "theme-radios",
      value: "light",
    }
    "Light"
  }
  label {
    input {
      r#type: "radio",
      name: "theme-radios",
      value: "dark",
    }
    "Dark"
  }
  label {
    input {
      r#type: "radio",
      name: "theme-radios",
      value: "cupcake",
    }
    "Cupcake"
  }
}
```

### Buttons
```rust
use wasm_theme::theme_buttons;

#[component]
fn App() -> Element {
  use_effect(move || {
    theme_buttons();
  });

  rsx! {
    ...
    SomeComponent {}
  }
}

#[component]
pub fn SomeComponent() -> Element {
  button {
    name: "theme-button",
    value: "light",
    "Light"
  }
  button {
    name: "theme-button",
    value: "dark",
    "Dark"
  }
  button {
    name: "theme-button",
    value: "cupcake",
    "Cupcake"
  }
}
```

### Select
```rust
use wasm_theme::theme_select;

#[component]
fn App() -> Element {
  use_effect(move || {
    theme_select();
  });

  rsx! {
    ...
    SomeComponent {}
  }
}

#[component]
pub fn SomeComponent() -> Element {
  select { name: "theme-select",
    option { value: "default", "Default" }
    option { value: "light", "Light" }
    option { value: "dark", "Dark" }
    option { value: "cupcake", "Cupcake" }
  }
}
