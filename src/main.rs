#[macro_use]
extern crate rouille;

use std::thread;
use std::net::{SocketAddrV4, Ipv4Addr, TcpListener};
use wry::{Application, Attributes, Result};


// The index file
static INDEX: &'static [u8] = include_bytes!("../wasm/index.html");
// The wasm code
static WASM: &'static [u8] = include_bytes!("../wasm/pkg/package_bg.wasm"); 
// The js code
static JSAPP: &'static [u8] = include_bytes!("../wasm/pkg/package.js");

// The js code does the WASM app loading, so both are needed

// the server that is gonna deliver the wasm application
fn serve(addr: String) {
    rouille::start_server(addr, move |request| {
        router!(request,
            (GET) ["/"] => {
                rouille::Response::from_data("text/html", INDEX)
            },
            (GET) ["/pkg/package.js"] => {
                rouille::Response::from_data("application/javascript", JSAPP)
            },
            (GET) ["/pkg/package_bg.wasm"] => {
                rouille::Response::from_data("application/wasm", WASM)
            },
            _ => rouille::Response::empty_404()
        )
    });
}

fn main() -> Result<()> {
    let socket = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0);
    let address = TcpListener::bind(socket)?.local_addr().unwrap();
    println!("{}", address.to_string());
    thread::spawn(move || {
        serve(address.to_string());
    });
    let mut app = Application::new()?;
    app.add_window(Attributes {
        title: "Rust WASM test".into(),
        url: Some(format!("http://{}", address.to_string())),
        debug: true,
        ..Default::default()
    }, None)?;
    app.run();
    Ok(())
}