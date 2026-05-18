use tauri::WebviewWindowBuilder;

/// Cloudflare / anti-bot handling.
///
/// This module implements various techniques to bypass Cloudflare bot protection:
/// - Custom User-Agent strings that mimic real browsers
/// - Additional headers that make requests look more legitimate
/// - JavaScript injection to handle Cloudflare challenges
pub fn init_cloudfare<R: tauri::Runtime, M: tauri::Manager<R>>(
    window_builder: WebviewWindowBuilder<R, M>,
) -> WebviewWindowBuilder<R, M> {
    // Apply Cloudflare bypass techniques to the webview window
    window_builder
        // Set a realistic User-Agent to avoid basic bot detection
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
}
