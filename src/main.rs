use pulldown_cmark::{html, Options, Parser};
use std::{env, fs, path::Path};
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::WebViewBuilder;

const CSS: &str = r#"
:root {
  --bg: #1e1e2e;
  --fg: #cdd6f4;
  --muted: #6c7086;
  --accent: #89b4fa;
  --green: #a6e3a1;
  --mauve: #cba6f7;
  --surface: #313244;
  --border: #45475a;
}
* { margin: 0; padding: 0; box-sizing: border-box; }
html {
  scroll-behavior: smooth;
}
body {
  font-family: "Inter", "Segoe UI", system-ui, -apple-system, sans-serif;
  font-size: 16px;
  line-height: 1.7;
  color: var(--fg);
  background: var(--bg);
  max-width: 760px;
  margin: 0 auto;
  padding: 48px 32px;
}
h1, h2, h3, h4, h5, h6 {
  color: var(--mauve);
  margin: 1.8em 0 0.6em;
  line-height: 1.3;
  font-weight: 600;
}
h1 { font-size: 2em; color: var(--mauve); border-bottom: 2px solid var(--border); padding-bottom: 0.3em; }
h2 { font-size: 1.5em; color: var(--accent); border-bottom: 1px solid var(--border); padding-bottom: 0.2em; }
h3 { font-size: 1.25em; }
h1:first-child { margin-top: 0; }
p { margin: 0.8em 0; }
a { color: var(--accent); text-decoration: none; }
a:hover { text-decoration: underline; }
strong { color: #f5c2e7; font-weight: 600; }
em { color: #f9e2af; }
code {
  font-family: "JetBrains Mono", "Fira Code", "Cascadia Code", monospace;
  font-size: 0.88em;
  background: var(--surface);
  color: var(--green);
  padding: 2px 6px;
  border-radius: 4px;
}
pre {
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: 8px;
  padding: 16px 20px;
  margin: 1em 0;
  overflow-x: auto;
}
pre code {
  background: none;
  padding: 0;
  font-size: 0.85em;
  line-height: 1.6;
}
blockquote {
  border-left: 3px solid var(--accent);
  padding: 0.5em 1em;
  margin: 1em 0;
  color: var(--muted);
  background: rgba(137, 180, 250, 0.05);
  border-radius: 0 6px 6px 0;
}
ul, ol { padding-left: 1.8em; margin: 0.6em 0; }
li { margin: 0.3em 0; }
li::marker { color: var(--accent); }
hr {
  border: none;
  border-top: 1px solid var(--border);
  margin: 2em 0;
}
table {
  width: 100%;
  border-collapse: collapse;
  margin: 1em 0;
}
th, td {
  border: 1px solid var(--border);
  padding: 8px 12px;
  text-align: left;
}
th { background: var(--surface); color: var(--accent); font-weight: 600; }
img { max-width: 100%; border-radius: 8px; }
::-webkit-scrollbar { width: 8px; }
::-webkit-scrollbar-track { background: var(--bg); }
::-webkit-scrollbar-thumb { background: var(--border); border-radius: 4px; }
::-webkit-scrollbar-thumb:hover { background: var(--muted); }
"#;

fn md_to_html(md: &str) -> String {
    let opts = Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_TABLES
        | Options::ENABLE_FOOTNOTES
        | Options::ENABLE_TASKLISTS;
    let parser = Parser::new_ext(md, opts);
    let mut body = String::new();
    html::push_html(&mut body, parser);

    format!(
        r#"<!DOCTYPE html>
<html><head>
<meta charset="utf-8">
<style>{CSS}</style>
</head><body>{body}</body></html>"#
    )
}

fn main() {
    let path = match env::args().nth(1) {
        Some(p) => p,
        None => {
            eprintln!("usage: mdlens <file.md>");
            std::process::exit(1);
        }
    };

    let md = fs::read_to_string(&path).unwrap_or_else(|e| {
        eprintln!("error: {e}");
        std::process::exit(1);
    });

    let title = Path::new(&path)
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| "mdlens".into());

    let html = md_to_html(&md);

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title(format!("mdr - {title}"))
        .with_inner_size(tao::dpi::LogicalSize::new(860.0, 700.0))
        .build(&event_loop)
        .unwrap();

    let builder = WebViewBuilder::new().with_html(&html);

    #[cfg(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "ios",
        target_os = "android"
    ))]
    let _webview = builder.build(&window).unwrap();

    #[cfg(not(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "ios",
        target_os = "android"
    )))]
    let _webview = {
        use tao::platform::unix::WindowExtUnix;
        use wry::WebViewBuilderExtUnix;
        let vbox = window.default_vbox().unwrap();
        builder.build_gtk(vbox).unwrap()
    };

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = ControlFlow::Exit;
        }
    });
}
