use std::borrow::Cow;

use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::{
    http::{header::CONTENT_TYPE, Response},
    WebViewBuilder,
};

// Embedded page produced by `wasm-pack build --target web --out-name package`.
static INDEX: &[u8] = include_bytes!("../wasm/index.html");
static JSAPP: &[u8] = include_bytes!("../wasm/pkg/package.js");
static WASM: &[u8] = include_bytes!("../wasm/pkg/package_bg.wasm");

fn main() -> wry::Result<()> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Rust WASM test")
        .build(&event_loop)
        .expect("failed to create window");

    // Custom protocol replaces the old 127.0.0.1 server. That server discovered
    // a free port by binding a socket and then closing it before rouille bound
    // the same port, so another process could take it first.
    let builder = WebViewBuilder::new()
        .with_devtools(true)
        .with_custom_protocol("wry".into(), |_id, request| asset_response(request.uri().path()))
        .with_url("wry://localhost/");

    #[cfg(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "ios",
        target_os = "android"
    ))]
    let _webview = builder.build(&window)?;
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
        builder.build_gtk(vbox)?
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

fn asset_response(path: &str) -> Response<Cow<'static, [u8]>> {
    let (body, mime) = match path {
        "/" | "" | "/index.html" => (INDEX, "text/html;charset=utf-8"),
        "/pkg/package.js" => (JSAPP, "text/javascript"),
        "/pkg/package_bg.wasm" => (WASM, "application/wasm"),
        _ => {
            return Response::builder()
                .status(404)
                .header(CONTENT_TYPE, "text/plain")
                .body(Cow::Borrowed(b"not found" as &[u8]))
                .unwrap();
        }
    };

    Response::builder()
        .header(CONTENT_TYPE, mime)
        .header("Access-Control-Allow-Origin", "*")
        .body(Cow::Borrowed(body))
        .unwrap()
}
