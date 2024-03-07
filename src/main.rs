use windows::{core::h, Foundation::Uri};

#[tokio::main]
async fn main() {
    // The URI to launch
    let uri_bing: Uri = Uri::CreateUri(h!("http://www.bing.com")).unwrap();

    // Launch the URI
    let success: bool = windows::System::Launcher::LaunchUriAsync(&uri_bing)
        .unwrap()
        .await
        .unwrap();

    if success {
        // URI launched
    } else {
        // URI launch failed
    }
}
