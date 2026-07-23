# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

- Fixed Changelog links

### Fixed

- Local storage access failures (private browsing, a sandboxed iframe
  without `allow-same-origin`, "block all storage" settings, etc.) no
  longer panic every `theme_*` function on load — the theme still applies
  for the current page load, it just won't persist across a reload.
- A control mistakenly missing the tag its selector implies (e.g. a stray
  non-`<input>` element with `name=theme-toggle`) no longer aborts wiring
  up every other correctly-tagged control on the page.
- `prefers_color_scheme`'s `prefers-color-scheme: light` branch was dead
  code in every real browser (the `dark` query always resolves `Ok(Some(_))`
  regardless of whether it matches, so the `light`/fallback `else` arms
  were unreachable) — simplified to the boolean check that was actually
  running.
- Calling a `theme_*` function more than once (a rerun `use_effect`, a
  remounted component) no longer attaches duplicate listeners to the same
  elements; already-wired controls are now skipped.

### Changed

- `prefers_color_scheme`'s result is now cached for the page's lifetime, so
  calling `theme_toggle`/`theme_radio`/`theme_buttons`/`theme_select`
  together (the documented usage) no longer redundantly re-reads local
  storage, re-evaluates `matchMedia`, and re-writes `data-theme` on each
  call.
- Removed the `Node`/`HtmlElement` `web-sys` features — unused by the
  library itself (moved to `[dev-dependencies.web-sys]`, where the test
  fixtures do need them).

### Added

- Unit tests for the toggle/checked-value parsing logic, and a
  `wasm-bindgen-test` browser suite covering `theme_toggle`'s resolved
  initial state, click behavior, and idempotent re-invocation.

## [0.1.1] - 2026-02-09

### Added

- CHANGLOG.md
- examples/dioxus/.cargo/config.toml

### Changed

- Updated Cargo.toml crate meta info
- Updated dependencies
- Updated example dependencies
- Updated .cargo/config.toml for better optimization

### Removed

- rust-version lock in Cargo.toml

[unreleased]: https://github.com/justins-engineering/wasm-theme/compare/v0.1.1...main
[0.1.1]: https://github.com/justins-engineering/wasm-theme/compare/v0.1.0...v0.1.1
