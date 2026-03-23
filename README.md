# mdlens

Minimal GUI markdown reader for Linux. Renders `.md` files in a native WebKitGTK webview with a clean dark theme.

## Install

**Dependencies (Fedora):**

```
sudo dnf install gtk3-devel webkit2gtk4.1-devel
```

**Build:**

```
cargo build --release
```

Binary at `target/release/mdlens`.

## Usage

```
mdlens file.md
```

## What it renders

Headings, bold, italic, inline code, code blocks, blockquotes, tables, ordered/unordered lists, task lists, footnotes, horizontal rules, links, images.

## Stack

- [pulldown-cmark](https://crates.io/crates/pulldown-cmark) — markdown parsing
- [wry](https://crates.io/crates/wry) + [tao](https://crates.io/crates/tao) — native webview window
- CSS — Catppuccin Mocha theme
