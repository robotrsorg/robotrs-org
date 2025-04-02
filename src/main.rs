use re_web_viewer_server::{WebViewerServer, WebViewerServerPort, WebViewerServerError};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

fn main() -> Result<(), WebViewerServerError> {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    let server = WebViewerServer::new("0.0.0.0", WebViewerServerPort::default())?;

    let server_url = server.server_url();
    println!("Server is running at: {}", server_url);

    println!("Waiting for Ctrl-C...");
    while running.load(Ordering::SeqCst) {}

    println!("Server stopping.");
    Ok(())
}
